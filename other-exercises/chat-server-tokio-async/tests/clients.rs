use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::test]
async fn clients() //-> io::Result<()>
{
    let mut join_handles = Vec::with_capacity(10);

    for i in 1..=10 {
        println!("Start connection of client {i}");

        join_handles.push(tokio::spawn(async move {
            let mut client = TcpStream::connect("127.0.0.1:6440")
                .await
                .expect("Connection of client {i} failed");
            println!("Client {i} connected");

            let mut buff = vec![0; 128];
            client
                .read(&mut buff)
                .await
                .expect("Read for client {i} failed");
            println!("Connection of client {i}: read the incoming data and saved on the buffer");

            let received =
                std::str::from_utf8(&buff[..]).expect("Conversion for utf8 for client {i} failed");
            println!("On client {i} got: {}", received);
        }));
    }

    for join_handle in join_handles {
        join_handle.await.unwrap();
    }
}
