use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync,
};

use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

use chat_server_tokio::Dispatch;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6440").await.unwrap();

    let (client_handler_tx, mut server_manager_rx) = sync::mpsc::channel::<Dispatch>(50);
    let (server_manager_tx, _client_manager_rx) = sync::broadcast::channel::<Dispatch>(50);

    let server_manager_tx_arc = Arc::new(Mutex::new(server_manager_tx));

    let server_manager_tx_arc_main = server_manager_tx_arc.clone();

    tokio::spawn(async move {
        println!("Start server message manager loop!");

        let server_prefix = "Server Manager";

        // TODO: implement unique identifications of users
        // let users: HashSet<String> = HashSet::new();

        loop {
            if let Some(dispatch) = server_manager_rx.recv().await {

                let processed_dispatch = match dispatch {
                    Dispatch::UserMessage { id, text } => Dispatch::UserMessage { id, text },
                    Dispatch::IdRequestResponse {
                        requested_id,
                        accepted: _,
                    } => {
                        let text = format!("{requested_id} entered the chat");
                        Dispatch::UserMessage { id : "SERVER".to_string() , text}
                    }
                };

                server_manager_tx_arc_main
                    .lock()
                    .expect("##The mutex of the arc to the server transmitter is poisoned: ")
                    .send(processed_dispatch)
                    .expect(r####"##The server transmitter could not send the dispatch: "####);

                println!(r###"{server_prefix}:> The dispatch was sent"###);
            }
        }
    });

    println!("Start server connection handler loop!");
    loop {
        let (stream, addr) = listener.accept().await?;

        let client_handler_tx = client_handler_tx.clone();
        let server_manager_tx_arc_secondary = server_manager_tx_arc.clone();

        let client_manager_rx = server_manager_tx_arc_secondary
            .lock()
            .expect("##The mutex of the arc to the server transmitter is poisoned: ")
            .subscribe();

        let buffer_incoming = Vec::with_capacity(256);

        tokio::spawn(async move {
            process(stream, addr, buffer_incoming, client_handler_tx, client_manager_rx).await;
        });
    }

    Ok(())
}

async fn process(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    mut buffer_incoming: Vec<u8>,
    client_handler_tx: sync::mpsc::Sender<Dispatch>,
    mut client_manager_rx: sync::broadcast::Receiver<Dispatch>,
) {
    let connection_prefix = format!("***---> Handler of {}", addr);

    println!("{}:> starting connection handler", connection_prefix);

    let connection_prefix_reader = Arc::new(connection_prefix);
    let connection_prefix_writer = connection_prefix_reader.clone();

    // TODO: implement unique identifications of users (here loop for username validation)

    let (shutdown_tx, shutdown_rx) = sync::oneshot::channel::<bool>();
    let (mut tcp_rd, mut tcp_wr) = io::split(stream);

    // task handling the tcp writer
    tokio::spawn(async move {

        let prefix = format!("{}; TCP-writer", connection_prefix_writer);
        loop {
            match client_manager_rx.recv().await {
                Ok(dispatch) => {
                    let serialised = serde_json::to_string(&dispatch).unwrap();

                    tcp_wr.write_all(serialised.as_bytes()).await.expect("##Error: ");

                }
                Err(_e) => {
                    println!(r##"{prefix}:> tcp_writer returned error, dropping it"##);
                    break;
                }
            }
        }



    });

    // handling the tcp reader
    let prefix = format!("{}; TCP-recv", *connection_prefix_reader);
    loop {
        if let Some(_) = shutdown_rx.await {
            break;
        }

        buffer_incoming.clear();
        match tcp_rd.read_buf(&mut buffer_incoming).await {
            Ok(n) => {
                if n > 0 {
                    let incoming_dispatch = serde_json::from_str(&String::from_utf8(buffer_incoming.clone()).unwrap()).unwrap();

                    client_handler_tx.send(incoming_dispatch).await.unwrap();
                    println!(r##"{prefix}:> msg sent to the server task"##);
                } else {
                    let final_msg = "Channel already closed";
                    let final_dispatch = Dispatch::UserMessage {id: "final".to_string(), text : "EXIT".to_string()};
                    client_handler_tx.send(final_dispatch).await.unwrap();
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
}
