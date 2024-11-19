use std::io::Write;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use chat_server_tokio::Message;
use serde_json;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Welcome to the KabanChat!");

    println!(
        "WARNING: this chat is still in its alfa version, it is therefore absolutely unsecure, \
    meaning that all that you write will be potentially readable by third parties."
    );
    println!();

    print!(r###"Please input your desired userid and press "Enter": "###);
    std::io::stdout().flush().expect("## Could not flush the stdout: ");

    let mut name = String::new();

    loop {
        std::io::stdin().read_line(&mut name).expect("## Could not read the stdin: ");
        let name = name.trim().to_string();

        if name.len() == 0 {
            print!("An empty name can't be accepted. Please, try input your name again: ");
            std::io::stdout().flush().expect("## Could not flush the stdout: ");
            continue;
        }
        break;
    }

    println!();

    let stream = TcpStream::connect("127.0.0.1:6440").await.expect("## Could not connect to the desired server: ");
    let (mut tcp_rd, mut tcp_wr) = tokio::io::split(stream);

    let mut incoming_buffer: Vec<u8> = Vec::with_capacity(256);
    let mut outgoing_buffer: Vec<u8> = Vec::with_capacity(256);

    // send "helo" message
    let helo_msg = Message::new(&name, "helo");
    let serialised = serde_json::to_string(&helo_msg).expect("## Could not serialize this string: ");

    tcp_wr.write_all(serialised.as_bytes()).await.expect("## TCP writer could not write: ");
    tcp_wr.flush().await.expect("## TCP writer could not flush: ");

    println!(
        r##"Please, {name}, write your messages and press "Enter" to send them. Type "EXIT" to quit the connection."##
    );


    // managing the tcp writer
    tokio::spawn(async move {
        loop {
            outgoing_buffer.clear();

            print!("\n{name}:> ");
            std::io::stdout().flush().expect("## Could not flush the stdout: ");

            tokio::io::stdin()
                .read_buf(&mut outgoing_buffer)
                .await
                .expect("## Could not read the buf in the stdin: ");

            // remove the last '\n'
            outgoing_buffer.pop();

            if outgoing_buffer.len() > 0 {
                let text = String::from_utf8(outgoing_buffer.clone()).expect("## Could not converst outbuffer to String: ");
                let outgoing_msg = Message::new(&name, &text);
                let serialised = serde_json::to_string(&outgoing_msg).expect("## Could not serialize this string: ");

                tcp_wr.write_all(serialised.as_bytes()).await.expect("## Could not write with TCP: ");
                tcp_wr.flush().await.expect("## Could not flush the TCP writer: ");
            }
        }
    });

    // managing the tcp reader
    loop {
        incoming_buffer.clear();
        let n = tcp_rd.read_buf(&mut incoming_buffer).await.expect("## Could not read the incoming buf: ");

        if n > 0 {
            let incoming_msg = Message::from_serialized_buffer(&incoming_buffer);
            println!("{incoming_msg}");
        } else {
            println!(r##"Closing the reading and writing tasks"##);
            break;
        }
    }
    Ok(())
}
