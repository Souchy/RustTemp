use std::error::Error;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::net::handler::MessageHandlers;
use crate::{Reader, Writer};

use super::Message;

#[derive(Clone)]
pub struct Client {
    reader: Reader,
    writer: Writer,
    handlers: Arc<MessageHandlers>,
}

impl Client {
    pub fn new(socket: TcpStream, handlers: Arc<MessageHandlers>) -> Self {
        let (r, w) = socket.into_split();
        let reader = Arc::new(Mutex::new(r));
        let writer = Arc::new(Mutex::new(w));
        Self {
            reader,
            writer,
            handlers,
        }
    }

    pub async fn new_connection(
        addr: &str,
        handlers: Arc<MessageHandlers>,
    ) -> Result<Self, Box<dyn Error>> {
        let socket = TcpStream::connect(addr).await?;
        Ok(Self::new(socket, handlers))
    }

    pub async fn send_bytes(&self, buf: &[u8]) -> Result<(), Box<dyn Error>> {
        self.writer.lock().await.write_all(buf).await?;
        return Ok(());
    }

    pub async fn send(&self, msg: Message) -> Result<(), Box<dyn Error>> {
        msg.send(self).await
    }

    pub async fn run(&self) -> Result<(), Box<dyn Error + Send>> {
        let mut buf = vec![0; 4 * 1024];
        loop {
            let n = self
                .reader
                .lock()
                .await
                .read(&mut buf)
                .await
                .expect("failed to read data from socket");

            println!("read {}", n);
            if n == 0 {
                println!("client connected terminated");
                break;
            }

            self.handlers
                .handle(&buf[0..n], self)
                .await
                .expect("message handling error");
        }
        Ok(())
    }
}
