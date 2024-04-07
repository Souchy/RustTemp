use std::error::Error;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::net::handler::MessageHandlers;
use crate::{Reader, Writer};

use super::message::MessageScript;
use super::Message;

#[derive(Clone)]
pub struct Client {
    reader: Reader,
    writer: Writer,
    // pipeline: Pipeline
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
        let result = self.writer.lock().await.write_all(buf).await;
        return Ok(result?);
    }

    pub async fn send(&self, msg: Message) -> Result<(), Box<dyn Error>> {
        msg.send(self).await
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn Error + Send>> {
        // println!("t1 start");
        let mut buf = vec![0; 4 * 1024];
        loop {
            let mut tr = self.reader.lock().await;
            // let mut tr = r;
            let n = tr
                .read(&mut buf)
                .await
                .expect("failed to read data from socket");

            // println!("read!: {}", n);
            if n == 0 {
                println!("client connected terminated");
                break;
            }

            // TODO: pipeline sucks, just use messageRegistry, but also we need a global Registry for all clients, maybe cloned from server, maybe Arc<>, it's not mutable
            // self.pipeline.handle(&buf);
            self.handlers.handle(&buf[0..n], self).await;
            // let msg = self.handlers.handle(&buf[0..n]);
            // if msg.is_some() {
            //     // // let re = Arc::new(self);
            //     let fds = msg.unwrap().handle(self).await;
            //     // self.handlers.deserialize(&buf[0..n]).handle(self).await;
            // } else {
            //     let st = std::str::from_utf8(&buf[0..n]).unwrap();
            //     println!("received: {}", st);
            //     self.writer.lock().await.write_all(b"pong").await.expect("msg");
            // }
        }
        Ok(())
    }
}
