use std::sync::Arc;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf};
use tokio::{
    net::{TcpListener, TcpStream},
    sync,
};

use chat_server_tokio::*;

#[tokio::main]
async fn main() -> Result<()> {
    let max_user_number = 200;
    assert!(max_user_number < u64::MAX);

    let server_address = "127.0.0.1:6440";
    let listener = TcpListener::bind(server_address).await?;

    println!("Welcome, the KabanChat server receives at maximum {max_user_number} users at the address {server_address}");

    let (client_handler_tx, mut server_manager_rx) = sync::mpsc::channel::<Dispatch>(50);
    let (server_manager_tx, _rx) = sync::broadcast::channel::<Dispatch>(50);
    let server_manager_tx_arc = Arc::new(server_manager_tx);

    let server_manager_mic = server_manager_tx_arc.clone();

    tokio::spawn(async move {
        let server_manager_name = "# [ServerManager]".to_string();

        println!("{server_manager_name}:> Starting ...");
        loop {
            if let Some(dispatch) = server_manager_rx.recv().await {
                server_manager_mic.send(dispatch)?;
            }
        }

        Ok::<(), GenericError>(())
    });

    println!("Start server connection handler loop!");

    let mut next_userid = 1_u64;

    while next_userid < max_user_number {
        let (stream, addr) = listener.accept().await?;

        let client_handler_tx = client_handler_tx.clone();
        let server_manager_subscriber = server_manager_tx_arc.clone();

        let client_handler_rx = server_manager_subscriber.subscribe();

        tokio::spawn(async move {
            process(
                stream,
                client_handler_tx,
                client_handler_rx,
                addr,
                next_userid,
            )
            .await?;
            Ok::<(), GenericError>(())
        });

        next_userid += 1;
    }

    /* // todo Review: idea: set containing all allowed get_userids. whenever a user connections, one of the
     get_userids is popped / removed from the set and the client will receive this one. once the set
     is empty, there's no more get_userids available -> error. otherwise, if a client disconnects,
     their get_userid becomes free again -> push it back to the set.*/
    println!(
        "The number of users (ever) connected reached the maximum supported quantity of {max_user_number}!"
    );
    println!("The server will thus now shut off!");

    Ok(())
}

async fn process(
    stream: TcpStream,
    client_handler_tx: sync::mpsc::Sender<Dispatch>,
    mut client_handler_rx: sync::broadcast::Receiver<Dispatch>,
    addr: std::net::SocketAddr,
    userid: u64,
) -> Result<()> {
    let task_id = format!("@ Handler of [{userid}:{addr}]");
    println!("{task_id}:> Starting...");
    let task_id_handle1 = Arc::new(task_id.clone());
    let task_id_handle2 = task_id_handle1.clone();

    let (mut tcp_rd, mut tcp_wr) = io::split(stream);

    // Review:: own function handling the tcp writer
    tokio::spawn(async move {
        let task_id = format!("{task_id_handle1} (TCP-writer)");

        loop {
            match client_handler_rx.recv().await {
                Ok(dispatch) => {
                    if dispatch.get_userid() != userid {
                        let serialised_msg = serde_json::to_string(&dispatch.into_msg())?;
                        tcp_wr.write_all(serialised_msg.as_bytes()).await?;
                    }
                }
                Err(e) => return Err::<(), GenericError>(GenericError::from(e)),
            }
        }
    });

    tcp_rd_loop(tcp_rd, task_id_handle2, client_handler_tx, userid).await?;

    Ok(())
}

async fn tcp_rd_loop(
    mut tcp_rd: ReadHalf<TcpStream>,
    supertask_id: Arc<String>,
    client_handler_tx: sync::mpsc::Sender<Dispatch>,
    userid: u64,
) -> Result<()> {
    let task_id = format!("{supertask_id} (TCP-recv)");

    let mut buffer_incoming = Vec::with_capacity(256);

    let username = get_username(&task_id, &mut tcp_rd, &mut buffer_incoming).await?;

    let init_dispatch = Dispatch::new(
        userid,
        Message::craft_status_change_msg(&username, UserStatus::Present),
    );

    client_handler_tx.send(init_dispatch).await?;

    println!(r##"{task_id}:> Handling {username}"##);

    loop {
        buffer_incoming.clear();

        match tcp_rd.read_buf(&mut buffer_incoming).await {
            Ok(n) => {
                if n > 0 {
                    let dispatch =
                        Dispatch::new(userid, Message::from_serialized_buffer(&buffer_incoming)?);

                    client_handler_tx.send(dispatch.clone()).await?;

                    println!("{task_id}:> {dispatch}");
                } else {
                    let final_dispatch = Dispatch::craft_status_change_dispatch(
                        userid,
                        &username,
                        UserStatus::Absent,
                    );
                    client_handler_tx.send(final_dispatch.clone()).await?;
                    println!("{task_id}:> {}", final_dispatch.into_msg());
                    break;
                }
            }
            // Review: https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read_buf
            // Ok(0) implies one of two possible scenarios, both could be treated separately (not needed here IMHO)
            Err(_e) => {
                println!("## The TCP-reader died: ");
                break;
            }
        };
    }
    Ok(())
}

async fn get_username(
    task_id: &str,
    tcp_rd: &mut ReadHalf<TcpStream>,
    buffer_incoming: &mut Vec<u8>,
) -> Result<String> {
    match tcp_rd.read_buf(buffer_incoming).await {
        Ok(n) if n > 0 => {
            let helo_msg = Message::from_serialized_buffer(&buffer_incoming)?;

            Ok(helo_msg.get_username())
        }
        Ok(_) | _ => {
            let err_msg = format!("{task_id}:> The 'helo' message was not received");
            let e = GenericError::from(io::Error::new(io::ErrorKind::NotFound, err_msg));
            Err(e)
        }
    }
}
