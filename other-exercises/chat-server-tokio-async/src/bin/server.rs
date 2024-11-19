use std::net::SocketAddr;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync;

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6440").await.unwrap();

    let (client_tx, mut server_rx) = sync::mpsc::channel(32);
    let (server_tx, _client_rx) = sync::broadcast::channel(32);

    let server_tx_arc = Arc::new(Mutex::new(server_tx));

    let server_tx_arc_main = server_tx_arc.clone();

    tokio::spawn(async move {
        println!("Start server loop!");
        loop {
            if let Some(message) = server_rx.recv().await {
                server_tx_arc_main
                    .lock()
                    .expect("The server transmitter is poisoned")
                    .send(message)
                    .unwrap();
            }
        }
    });

    loop {
        let (socket, addr) = listener.accept().await?;

        let client_tx = client_tx.clone();

        let server_tx_arc_secundary = server_tx_arc.clone();
        let client_rx = server_tx_arc_secundary
            .lock()
            .expect("The server transmitter is poisoned")
            .subscribe();
        let buffer = String::with_capacity(32);

        tokio::spawn(async move {
            process(socket, addr, buffer, client_tx, client_rx).await;
        });
    }

    Ok(())
}

async fn process(
    socket: TcpStream,
    addr : SocketAddr,
    mut buffer: String,
    client_tx: sync::mpsc::Sender<String>,
    mut client_rx: sync::broadcast::Receiver<String>,
) {
    let connection_name = format!("\tConnection {}", addr);
    println!("{connection_name}:> starting connection");

    let (mut rd, mut wr) = io::split(socket);

    loop {
        buffer.clear();
        if let Ok(n) = rd.read_to_string(&mut buffer).await {
            if n > 0 {
                println!("{connection_name}:> received {n} measure units of text!!");
                client_tx.send(buffer.to_string().clone()).await.unwrap();
            }
        }
        
        if let message = client_rx.recv().await.unwrap() {
            wr.write_all(message.as_bytes())
                .await
                .expect("There was an error with the writing process");
            let message = message.trim();
            println!(r##"{connection_name}:> msg "{message}" sent through the reader"##);
        }
    }
}
