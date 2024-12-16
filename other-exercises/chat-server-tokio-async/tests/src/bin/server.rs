use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync,
};

use std::sync::{Arc, Mutex};

use chat_server_tokio::{Dispatch, Message};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6440").await.unwrap();

    let (client_handler_tx, mut server_manager_rx) = sync::mpsc::channel::<Dispatch>(50);
    let (server_manager_tx, _client_manager_rx) = sync::broadcast::channel::<Dispatch>(50);

    let server_manager_tx_arc = Arc::new(Mutex::new(server_manager_tx));

    let server_manager_tx_arc_main = server_manager_tx_arc.clone();

    tokio::spawn(async move {
        let server_manager_name = "# [ServerManager]".to_string();

        println!("{server_manager_name}:> Starting ...");
        loop {
            if let Some(dispatch) = server_manager_rx.recv().await {
                server_manager_tx_arc_main
                    .lock()
                    .expect("## The mutex of the arc to the server transmitter is poisoned: ")
                    .send(dispatch)
                    .expect("## The server transmitter could not send the dispatch: ");
            }
        }
    });

    println!("Start server connection handler loop!");

    // next_userid
    let mut usercount = 1_u64;

    while usercount < u64::MAX {
        // TODO: nachschauen, ob auf dem server einfach mehrere sockets für die gleiche ip und port
        // aufgebaut werden, oder ob per ip:port lediglich auf einen anderen port verwiesen wird,
        // über welchen die kommunikation abläuft
        let (stream, addr) = listener.accept().await?;

        let client_handler_tx = client_handler_tx.clone();
        let server_manager_tx_arc_secondary = server_manager_tx_arc.clone();

        // Review: lock really necessary? why does it work with this being locked?
        // maybe try to put in a sleep, and try to make multiple senders send
        let client_manager_rx = server_manager_tx_arc_secondary
            .lock()
            .expect("## The mutex of the arc to the server transmitter is poisoned: ")
            .subscribe();

        tokio::spawn(async move {
            process(
                stream,
                addr,
                usercount,
                client_handler_tx,
                client_manager_rx,
            )
            .await;
        });

        usercount += 1;
    }

    // Review: idea: set containing all allowed uids. whenever a user connections, one of the
    // uids is popped / removed from the set and the client will receive this one. once the set
    // is empty, there's no more uids available -> error. otherwise, if a client disconnects,
    // their uid becomes free again -> push it back to the set
    // todo: handle this with a set (to have userid uniqueness) with a maximum fixed capacity,
    // so that if a user disconnects, he is NOT still counted among the users raising the count
    println!(
        "The number of users (ever) connected reached the supported quantity of {}!",
        u64::MAX
    );
    println!("The server will thus now shut off!");

    Ok(())
}

// Review: I am not sure if this is the right way to go that you spawn a task for process, and
// process itself also spawns more tasks itself. maybe it's cleaner to define and call two
// methods process_reader and process_writer (or with better names) separately, so each accepted
// connection in main will lead to two spawned tasks itself, rather than this being done by a
// spawned subtask.
async fn process(
    stream: TcpStream,
    addr: std::net::SocketAddr,
    userid: u64,
    client_handler_tx: sync::mpsc::Sender<Dispatch>,
    mut client_manager_rx: sync::broadcast::Receiver<Dispatch>,
) {
    // Review: see https://docs.rs/tokio/latest/tokio/net/struct.TcpListener.html
    // in the description for the accept function it is stated, that the addr corresponds to the
    // remote peer (client) address. I am pretty sure that the server can accept multiple
    // connections for the same ip:port pair via different sockets. this makes sense, since the
    // tcp-stream opened in the main function is kept alive here throughout the whole time and
    // doesn't out of scope. that's just for clarification
    let handler_name = format!("@ Handler of [{userid}]-[{addr}]");

    println!("{}:> Starting...", handler_name);

    let handler_name_reader = Arc::new(handler_name);
    let handler_name_writer = handler_name_reader.clone();

    let (mut tcp_rd, mut tcp_wr) = io::split(stream);

    // task handling the tcp writer
    // Review:: own function
    tokio::spawn(async move {
        let prefix = format!("{handler_name_writer} (TCP-writer)");

        loop {
            match client_manager_rx.recv().await {
                Ok(dispatch) => {
                    if dispatch.uid() != userid {
                        let serialised_msg = serde_json::to_string(&dispatch.extract_msg())
                            .expect("## The serialization process went wrong: ");
                        tcp_wr
                            .write_all(serialised_msg.as_bytes())
                            .await
                            .expect("## The TCP-writer died: ");
                    }
                }
                Err(_e) => {
                    println!(r##"{prefix}:> tcp_writer returned error, dropping it"##);
                    break;
                }
            }
        }
    });

    // handling the tcp reader
    let prefix = format!("{handler_name_reader} (TCP-recv)");

    let mut buffer_incoming = Vec::with_capacity(256);

    // by protocol the first msg is a "helo"
    let mut username = String::new();

    // ======================================================= task 2
    // Review: own function verify_helo
    match tcp_rd.read_buf(&mut buffer_incoming).await {
        Ok(n) if n > 0 => {
            let helo_msg = Message::from_serialized_buffer(&buffer_incoming);

            username = helo_msg.uname();

            let init_dispatch = Dispatch::new(userid, Message::craft_ack_msg(&username));

            client_handler_tx
                .send(init_dispatch.clone()) // Review: don't clone
                .await
                .expect("## The handler could not transmit the dispatch: ");
        }
        Ok(_) | _ => println!("## No helo message received!!!"),
    };

    println!(r##"{prefix}:> Handling {username}"##);

    loop {
        buffer_incoming.clear();

        match tcp_rd.read_buf(&mut buffer_incoming).await {
            Ok(n) => {
                if n > 0 {
                    let dispatch =
                        Dispatch::new(userid, Message::from_serialized_buffer(&buffer_incoming));

                    client_handler_tx
                        .send(dispatch.clone())
                        .await
                        .expect("## The handler could not transmit the dispatch: ");

                    println!("{prefix}:> {dispatch}");
                } else {
                    let final_dispatch = Dispatch::craft_final_dispatch(userid, &username);
                    client_handler_tx
                        .send(final_dispatch.clone())
                        .await
                        .expect("## The handler could not transmit the dispatch: ");
                    println!("{prefix}:> {}", final_dispatch.extract_msg());
                    break;
                }
            }
            // Review: see read_buf in https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_buf
            // Ok(0) implies one of two possible scenarios, both could be treated separately (not needed here IMHO)
            Err(_e) => {
                println!("## The TCP-reader died: ");
                break;
            }
        };
    }
}
