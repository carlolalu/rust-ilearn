use std::io::Write;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::io;

use chat_server_tokio::Dispatch;
use serde_json;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Welcome to the KabanChat!");

    println!(
        "WARNING: this chat is still in its alfa version, it is therefore absolutely unsecure, \
    meaning that all that you write will be potentially readable by third parties."
    );
    println!();

    let stream = TcpStream::connect("127.0.0.1:6440").await.unwrap();
    let (mut tcp_rd, mut tcp_wr) = tokio::io::split(stream);

    let mut incoming_buffer: Vec<u8> = Vec::with_capacity(256);
    let mut outgoing_buffer: Vec<u8> = Vec::with_capacity(256);

    print!(r###"Please input your desired userid and press "Enter": "###);
    std::io::stdout().flush().unwrap();

    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    println!();

    // TODO: handle the protocol granting uniqueness of usernames (here loopp to ask usernameto the server)

    println!(r##"Please, {name}, write your messages and press "Enter" to send them. Type "EXIT" to quit the connection."##);

    // managing the tcp writer
    tokio::spawn(async move {
        loop {
            outgoing_buffer.clear();

            print!("\n{name}:> ");
            std::io::stdout().flush().unwrap();

            tokio::io::stdin().read_buf(&mut outgoing_buffer).await.unwrap();

            outgoing_buffer.pop();

            if outgoing_buffer.len() > 0 {
                let text = String::from_utf8(outgoing_buffer.clone()).unwrap();
                let outgoing_dispatch = Dispatch::UserMessage {id : name.clone(), text};
                let serialised = serde_json::to_string(&outgoing_dispatch).unwrap();

                tcp_wr.write_all(serialised.as_bytes()).await.unwrap();
                tcp_wr.flush().await.unwrap();
            }
        }
    });

    // managing the tcp reader
    loop {
        incoming_buffer.clear();
        let n = tcp_rd.read_buf(&mut incoming_buffer).await.unwrap();

        if n > 0 {
            let incoming_dispatch : Dispatch = serde_json::from_str(&String::from_utf8(incoming_buffer.clone()).unwrap()).unwrap();
            println!("{incoming_dispatch}");
        } else {
            println!(r##"Closing the reading and writing tasks"##);
            break;
        }
    }
    Ok(())
}
