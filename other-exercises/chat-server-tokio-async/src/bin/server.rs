use std::sync::Arc;

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::{TcpListener, TcpStream},
    sync,
};

use tokio_util::{sync::CancellationToken, task::TaskTracker};

use chat_server_tokio::*;

static MAX_NUM_USER: u64 = 2000;

#[tokio::main]
async fn main() -> Result<()> {
    assert!(MAX_NUM_USER < u64::MAX);

    println!("Welcome, the KabanChat server receives at maximum {MAX_NUM_USER} users at the address {SERVER_ADDR}");

    let (client_handler_tx, dispatcher_rx) = sync::mpsc::channel::<Dispatch>(50);
    let (dispatcher_tx, _rx) = sync::broadcast::channel::<Dispatch>(50);

    let dispatcher_tx_arc = Arc::new(dispatcher_tx);
    let dispatcher_mic = dispatcher_tx_arc.clone();

    let main_tracker = TaskTracker::new();

    // in this two I have done surely an error, but how? Why do I have two results wrapped in one another? The green_threads return a result as well: it is Err if the thread panicked. Still, is this then not a correct procedure? Should I await inside an async move block? Or should I proceed?
    main_tracker.spawn(async move {
        dispatcher(dispatcher_rx, dispatcher_mic).await?;
        Ok::<(), GenericError>(())
    });

    main_tracker.spawn(async move {
        server_manager(dispatcher_tx_arc, client_handler_tx).await?;
        Ok::<(), GenericError>(())
    });

    main_tracker.close();
    main_tracker.wait().await;

    Ok(())
}

/// This function is responsible of passing dispatches between client_handlers
async fn dispatcher(
    mut dispatcher_rx: sync::mpsc::Receiver<Dispatch>,
    dispatcher_mic: Arc<sync::broadcast::Sender<Dispatch>>,
) -> Result<()> {
    let dispatcher_name = "# [Dispatcher]".to_string();

    println!("{dispatcher_name}:> Starting ...");
    loop {
        if let Some(dispatch) = dispatcher_rx.recv().await {
            dispatcher_mic.send(dispatch)?;
        }
    }

    Ok::<(), GenericError>(())
}



/// This function creates a socket and accepts connections on it, spawining a task for each connection
async fn server_manager(
    dispatcher_tx_arc: Arc<sync::broadcast::Sender<Dispatch>>,
    client_handler_tx: sync::mpsc::Sender<Dispatch>,
) -> Result<()> {
    println!("Start server manager loop!");

    let listener = TcpListener::bind(SERVER_ADDR).await?;

    let server_manager_tracker = TaskTracker::new();

    let mut next_userid = 1_u64;

    while next_userid < MAX_NUM_USER {
        let (stream, addr) = listener.accept().await?;

        let client_handler_tx = client_handler_tx.clone();
        let dispatcher_subscriber = dispatcher_tx_arc.clone();

        let client_handler_rx = dispatcher_subscriber.subscribe();

        server_manager_tracker.spawn(async move {
            client_handler(
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

    server_manager_tracker.close();
    server_manager_tracker.wait().await;

    /* // todo Review: idea: set containing all allowed get_userids. whenever a user connections, one of the
    get_userids is popped / removed from the set and the client will receive this one. once the set
    is empty, there's no more get_userids available -> error. otherwise, if a client disconnects,
    their get_userid becomes free again -> push it back to the set.*/
    println!(
        "The number of users (ever) connected reached the maximum supported quantity of {MAX_NUM_USER}!"
    );
    println!("The server will thus now shut off!");

    Ok::<(), GenericError>(())
}

async fn client_handler(
    stream: TcpStream,
    client_handler_tx: sync::mpsc::Sender<Dispatch>,
    client_handler_rx: sync::broadcast::Receiver<Dispatch>,
    addr: std::net::SocketAddr,
    userid: u64,
) -> Result<()> {
    let task_id = format!("@ Handler of [{userid}:{addr}]");
    let client_handler_task_manager = TaskTracker::new();

    println!("{task_id}:> Starting...");
    let task_id_handle1 = Arc::new(task_id.clone());
    let task_id_handle2 = task_id_handle1.clone();

    let (tcp_rd, tcp_wr) = io::split(stream);



    // Review:: own function handling the tcp writer
    client_handler_task_manager.spawn(async move {
        client_tcp_wr_loop(tcp_wr, task_id_handle1, client_handler_rx, userid).await?;
        Ok::<(), GenericError>(())
    });

    client_handler_task_manager.spawn(async move{
        client_tcp_rd_loop(tcp_rd, task_id_handle2, client_handler_tx, userid).await?;
        Ok::<(), GenericError>(())
    });


    // Now I dunno yet how to use such task tracer, and thus I just implement this for each one task that has subtasks.
    client_handler_task_manager.close();
    client_handler_task_manager.wait().await;


    Ok(())
}


async fn client_tcp_wr_loop(
    mut tcp_wr: WriteHalf<TcpStream>,
    supertask_id: Arc<String>,
    mut client_handler_rx: sync::broadcast::Receiver<Dispatch>,
    userid: u64,
) -> Result<()> {
    let task_id = format!("{supertask_id} (TCP-writer)");

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

    Ok(())
}


async fn client_tcp_rd_loop(
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
