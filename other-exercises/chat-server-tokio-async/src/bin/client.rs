use std::io::Write;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::{io, sync};

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("Welcome to the KabanChat!");

    println!("WARNING: this chat is still in its alfa version, it is therefore absolutely unsecure, \
    meaning that all that you write will be potentially readable by third parties.");

    print!("Input your name please: ");
    std::io::stdout().flush().unwrap();

    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    println!();

    let stream = TcpStream::connect("127.0.0.1:6440").await.unwrap();
    let (mut tcp_rd, mut tcp_wr) = tokio::io::split(stream);

    let mut incoming: Vec<u8> = Vec::with_capacity(50);
    let mut outgoing: Vec<u8> = Vec::with_capacity(50);

    println!(r##"Please, {name}, write your messages and press "Enter" to send them."##);
    println!();

    // managing the tcp reader
    tokio::spawn(async move {
        loop {
            incoming.clear();
            let n = tcp_rd.read_buf(&mut incoming).await.unwrap();

            if n > 0 {
                println!("\nserver:> {}", String::from_utf8(incoming.clone()).unwrap());
            } else {
                println!(r##"ATTENTION: no data reaad, closing the reading task"##);
                break;
            }
        }
    });

    // managing the tcp writer
    loop {
        outgoing.clear();
        print!("\n{name}:> ");
        std::io::stdout().flush().unwrap();

        tokio::io::stdin().read_buf(&mut outgoing).await.unwrap();

        outgoing.pop();

        if outgoing.len() > 0 {
            tcp_wr.write_all(&outgoing[..]).await.unwrap();
            tcp_wr.flush().await.unwrap();
        }
    }

    Ok(())
}
