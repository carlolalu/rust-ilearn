use tokio::net::TcpStream;
use mini_redis::{Frame, Result};

struct Connection {
    stream: TcpStream,
}

impl Connection {
    /// Read a frame from the connection.
    ///
    /// Returns `None` if EOF is reached
    pub async fn read_frame(&mut self)
                            -> Result<Option<Frame>>
    {
        // implementation here
    }

    /// Write a frame to the connection.
    pub async fn write_frame(&mut self, frame: &Frame)
                             -> Result<()>
    {
        // implementation here
    }
}