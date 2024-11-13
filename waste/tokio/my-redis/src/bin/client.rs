use bytes::Bytes;
use tokio::sync::mpsc;
use mini_redis::client;
use tokio::sync::oneshot;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    let (tx1, mut rx) = mpsc::channel(32);

    let resources_manager = tokio::spawn( async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp} => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                },
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    let _ = resp.send(res);
                },
            }
        }
    });

    let tx2 = tx1.clone();

    let t1 = tokio::spawn( async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "foo".into(),
            resp: resp_tx,
        };

        tx1.send(cmd).await.unwrap();

        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn( async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: "foo".into(),
            val: "bar".into(),
            resp: resp_tx,
        };

        tx2.send(cmd).await.unwrap();
        let res = resp_rx.await;
        println!("GOT = {:?}", res);

    });

    t1.await.unwrap();
    t2.await.unwrap();
    resources_manager.await.unwrap();
}