use std::io::Write;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::{tcp, TcpStream};

use chat_server_tokio::{GenericError, Message, Result, SERVER_ADDR};
use serde_json;
use tokio_util::task::TaskTracker;

#[tokio::main]
async fn main() -> Result<()> {

    println!("========================");
    println!("Welcome to the KabanChat!");
    println!(
        "WARNING: this chat is still in its alfa version, it is therefore absolutely unsecure, meaning that all that you write will be potentially readable by third parties."
    );
    println!("========================");
    println!();

    let (tcp_rd, tcp_wr, name) = connect_and_login().await?;

    println!(
        r##"Please, {name}, write your messages and press "Enter" to send them. Type "EXIT" to quit the connection."##
    );

    
    let main_tracker = TaskTracker::new();

    main_tracker.spawn(async move {
        wr_manager(tcp_wr, &name).await?;

        Ok::<(), GenericError>(())
    });

    main_tracker.spawn(async move {
        rd_manager(tcp_rd).await?;

        Ok::<(), GenericError>(())
    });

    main_tracker.close();

    main_tracker.wait().await;

    Ok(())
}

async fn connect_and_login() -> Result<(ReadHalf<TcpStream>, WriteHalf<TcpStream>, String)> {
    print!(r###"Please input your desired userid and press "Enter": "###);
    std::io::stdout().flush()?;

    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    let stream = TcpStream::connect(SERVER_ADDR).await?;
    let (tcp_rd, mut tcp_wr) = tokio::io::split(stream);

    // send "helo" message
    let serialised_helo_msg = serde_json::to_string(&Message::new(&name, "helo"))?;

    tcp_wr.write_all(serialised_helo_msg.as_bytes()).await?;
    tcp_wr.flush().await?;

    Ok((tcp_rd, tcp_wr, name.to_string()))
}

async fn wr_manager(mut tcp_wr: WriteHalf<TcpStream>, name: &str) -> Result<()> {
    let mut outgoing_buffer: Vec<u8> = Vec::with_capacity(256);

    loop {
        outgoing_buffer.clear();

        print!("\n{name}:> ");
        std::io::stdout().flush()?;

        tokio::io::stdin().read_buf(&mut outgoing_buffer).await?;

        // remove the last '\n'
        outgoing_buffer.pop();

        if outgoing_buffer.len() > 0 {
            let text = String::from_utf8(outgoing_buffer.clone())?;
            let outgoing_msg = Message::new(&name, &text);
            let serialised = serde_json::to_string(&outgoing_msg)?;

            tcp_wr.write_all(serialised.as_bytes()).await?;
        }
    }

    Ok::<(), GenericError>(())
}

async fn rd_manager(mut tcp_rd: ReadHalf<TcpStream>) -> Result<()> {
    let mut incoming_buffer: Vec<u8> = Vec::with_capacity(256);

    loop {
        incoming_buffer.clear();
        // Review: immediately call match on it
        let n = tcp_rd.read_buf(&mut incoming_buffer).await?;

        if n > 0 {
            let incoming_msg = Message::from_serialized_buffer(&incoming_buffer)?;
            println!("{incoming_msg}");
        } else {
            println!(r##"Closing the reading and writing tasks"##);
            break;
        }
    }
    Ok::<(), GenericError>(())
}
