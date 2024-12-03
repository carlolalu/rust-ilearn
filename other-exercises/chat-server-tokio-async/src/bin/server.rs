use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync;

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6440").await.unwrap();

    let (client_tx, mut server_rx) = sync::mpsc::channel(50);
    let (server_tx, _client_rx) = sync::broadcast::channel(50);

    let server_tx_arc = Arc::new(Mutex::new(server_tx));

    let server_tx_arc_main = server_tx_arc.clone();

    tokio::spawn(async move {
        println!("Start server loop!");
        loop {
            if let Some(message) = server_rx.recv().await {
                println!("The server is transmitting the msg '{message}'!");
                server_tx_arc_main
                    .lock()
                    .expect("##The server transmitter is poisoned: ")
                    .send(message)
                    .unwrap();
            }
        }
    });

    loop {
        let (stream, addr) = listener.accept().await?;

        let client_tx = client_tx.clone();

        let server_tx_arc_secundary = server_tx_arc.clone();
        let client_rx = server_tx_arc_secundary
            .lock()
            .expect("##The server transmitter is poisoned: ")
            .subscribe();
        let buffer = Vec::with_capacity(50);

        tokio::spawn(async move {
            process(stream, addr, buffer, client_tx, client_rx).await;
        });
    }
    Ok(())
}

async fn process(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    mut buffer: Vec<u8>,
    client_tx: sync::mpsc::Sender<String>,
    mut client_rx: sync::broadcast::Receiver<String>,
) {
    let connection_prefix = format!("\t{}", addr);

    let connection_prefix = std::sync::Arc::new(connection_prefix);
    let connection_prefix2 = connection_prefix.clone();

    println!("{connection_prefix}:> starting connection");

    let (mut tcp_rd, mut tcp_wr) = io::split(stream);

    // task handling the
    tokio::spawn(async move {
        loop {
            buffer.clear();
            if let Ok(n) = tcp_rd.read_buf(buffer).await {
                if n > 0 {
                    println!("{}:> received {n} measure units of text!", *connection_prefix);
                    client_tx.send(String::from_utf8(buffer).unwrap().clone()).await.unwrap();
                    println!(r##"{}:> msg sent to the server task"##, *connection_prefix);
                }
            }
        }
    });

    tokio::spawn(async move {
        loop {
            let message = client_rx.recv().await.unwrap();
            if !message.is_empty() {
                let message = message.trim().to_string();
                println!(r##"{}:> '{message}' was received by the server and will be transmitted to the client"##, *connection_prefix2);
                tcp_wr.write_all(message.as_bytes())
                    .await
                    .expect("##Error: ");
            }
        }
    });
}
