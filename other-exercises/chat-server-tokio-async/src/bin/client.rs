use std::io::{Read, Write};
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

use chat_server_tokio::{GenericError, Message, Result};
use serde_json;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to the KabanChat!");
    println!(
        "WARNING: this chat is still in its alfa version, it is therefore absolutely unsecure, \
    meaning that all that you write will be potentially readable by third parties."
    );
    println!();

    let server_addr = "127.0.0.1:6440";

    print!(r###"Please input your desired userid and press "Enter": "###);
    std::io::stdout().flush()?;

    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();

    let (mut tcp_rd, mut tcp_wr) = login(server_addr, &name).await?;

    let mut incoming_buffer: Vec<u8> = Vec::with_capacity(256);
    let mut outgoing_buffer: Vec<u8> = Vec::with_capacity(256);

    // send "helo" message
    let helo_msg = Message::new(&name, "helo");
    let serialised =
        serde_json::to_string(&helo_msg)?;

    tcp_wr.write_all(serialised.as_bytes()).await?;
    tcp_wr.flush().await?;

    println!(
        r##"Please, {name}, write your messages and press "Enter" to send them. Type "EXIT" to quit the connection."##
    );

    // Review: own function maybe?
    // managing the tcp writer
    tokio::spawn(async move {
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

                tcp_wr
                    .write_all(serialised.as_bytes())
                    .await
                    .expect("## Could not write with TCP: ");
                tcp_wr
                    .flush()
                    .await
                    .expect("## Could not flush the TCP writer: "); // Review: why is the flush required?
            }
        }

        Ok::<(),GenericError>(())
    });

    // managing the tcp reader
    loop {
        incoming_buffer.clear();
        // Review: immediately call match on it
        let n = tcp_rd
            .read_buf(&mut incoming_buffer)
            .await
            .expect("## Could not read the incoming buf: ");

        if n > 0 {
            let incoming_msg = Message::from_serialized_buffer(&incoming_buffer)?;
            println!("{incoming_msg}");
        } else {
            println!(r##"Closing the reading and writing tasks"##);
            break;
        }
    }
    Ok(())
}

async fn login(
    server_addr: &str,
    name: &str,
) -> Result<(ReadHalf<TcpStream>, WriteHalf<TcpStream>)> {
    let stream = TcpStream::connect(server_addr).await?;
    let (mut tcp_rd, mut tcp_wr) = tokio::io::split(stream);

    Ok((tcp_rd, tcp_wr))
}
