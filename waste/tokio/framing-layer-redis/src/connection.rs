use bytes::BytesMut;
use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use bytes::Buf;
use mini_redis::Result;

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            // Allocate the buffer with 4kb of capacity.
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // 0 = EOF
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }
}