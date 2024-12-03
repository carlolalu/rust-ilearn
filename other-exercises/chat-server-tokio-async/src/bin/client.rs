use tokio::io::{AsyncReadExt, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::io::Write;
use tokio::net::tcp::WriteHalf;


fn main() {
    println!("Welcome to the KabanChat!");

    print!("Input your name please: ");
    std::io::stdout().flush().unwrap();

    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();

    println!();

    println!("dbg point 1");

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let stream = TcpStream::connect("127.0.0.1:6440").await.unwrap();
            let (mut tcp_rd, mut tcp_wr) = tokio::io::split(stream);

            println!("dbg point 2");

            let mut incoming = String::with_capacity(32);
            let mut outgoing = String::new();

            println!(r##"Please, {name}, write your messages and press "Enter" to send them."##);
            println!();

            let mut reader = tokio::io::BufReader::new(tokio::io::stdin());

            tokio::spawn(async move {
                println!("dbg point 2.1");
                loop {
                    println!("dbg point 2.2");
                    let n = tcp_rd.read_until(b'\n', incoming).await.unwrap();
                    //if let Ok(n) = rd.read_to_string(&mut incoming).await {
                        println!("dbg point 2.3");
                        if n > 0 {
                            println!("dbg point 2.5");
                            println!("{}", incoming);
                        }
                    //}
                }
            });

            tokio::spawn(async move {
                loop {
                    outgoing.clear();

                    print!("{name}:> ");
                    std::io::stdout().flush().unwrap();

                    reader.read_line(&mut outgoing).await.unwrap();

                    println!("dbg point 3");

                    tcp_wr.write_all(outgoing.trim().as_bytes()).await.unwrap();
                    tcp_wr.flush().await.unwrap();
                }
            });

            loop {}
        });
}
