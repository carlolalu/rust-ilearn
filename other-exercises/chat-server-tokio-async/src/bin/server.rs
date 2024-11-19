use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync,
};

use std::sync::{Arc, Mutex};


#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6440").await.unwrap();

    let (client_handler_tx, mut server_manager_rx) = sync::mpsc::channel::<Vec<u8>>(50);
    let (server_manager_tx, _client_manager_rx) = sync::broadcast::channel::<Vec<u8>>(50);

    let server_manager_tx_arc = Arc::new(Mutex::new(server_manager_tx));

    let server_manager_tx_arc_main = server_manager_tx_arc.clone();

    tokio::spawn(async move {

        println!("Start server message manager loop!");

        loop {
            if let Some(bytes) = server_manager_rx.recv().await {
                let message = String::from_utf8(bytes.clone()).unwrap();

                println!("The server is transmitting the msg '{message}'!");

                server_manager_tx_arc_main
                    .lock()
                    .expect("##The server transmitter is poisoned: ")
                    .send(bytes)
                    .unwrap();
            }
        }
    });

    println!("Start server connection handler loop!");
    loop {
        let (stream, addr) = listener.accept().await?;

        let client_handler_tx = client_handler_tx.clone();

        let server_manager_tx_arc_secundary = server_manager_tx_arc.clone();

        let client_manager_rx = server_manager_tx_arc_secundary
            .lock()
            .expect("##The server transmitter is poisoned: ")
            .subscribe();

        let buffer = Vec::with_capacity(50);

        tokio::spawn(async move {
            process(stream, addr, buffer, client_handler_tx, client_manager_rx).await;
        });
    }

    Ok(())
}

async fn process(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    mut buffer: Vec<u8>,
    client_handler_tx: sync::mpsc::Sender<Vec<u8>>,
    mut client_manager_rx: sync::broadcast::Receiver<Vec<u8>>,
) {
    let connection_prefix = format!("  *-> Handler of {}", addr);

    let connection_prefix = std::sync::Arc::new(connection_prefix);
    let connection_prefix2 = connection_prefix.clone();

    println!("{connection_prefix}:> starting connection");

    let (mut tcp_rd, mut tcp_wr) = io::split(stream);

    // task handling the tcp receiver
    tokio::spawn(async move {
        let prefix = format!("{}; TCP-recv", *connection_prefix);
        loop {
            buffer.clear();
            match tcp_rd.read_buf(&mut buffer).await {
                Ok(n) => {
                    if n > 0 {
                        println!("{prefix}:> received {n} measure units of text!");
                        client_handler_tx.send(buffer.clone()).await.unwrap();
                        println!(r##"{prefix}:> msg sent to the server task"##);
                    } else {
                        let final_msg = "msg of 0 length read, dropping the reader task";
                        client_handler_tx.send(Vec::from(final_msg)).await.unwrap();
                        println!("{prefix}:> {final_msg}");
                        break;
                    }
                }
                Err(_e) => {
                    println!(r##"{prefix}:> returned error, dropping the task"##);
                    break;
                }
            };
        }
    });

    // handling the tcp writer
    let prefix = format!("{}; TCP-writer", *connection_prefix2);
    loop {
        match client_manager_rx.recv().await {
            Ok(bytes) => {
                if !bytes.is_empty() {
                    let message = String::from_utf8(bytes.clone()).unwrap();
                    println!(
                        r##"{prefix}:> '{message}' was received by the server and will be transmitted to the client"##
                    );

                    tcp_wr.write_all(&bytes[..]).await.expect("##Error: ");
                }
            }
            Err(_e) => {
                println!(r##"{prefix}:> tcp_writer returned error, dropping it"##);
                break;
            }
        }
    }
}
