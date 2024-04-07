use std::error::Error;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::{net::TcpListener, sync::Mutex};

use crate::{Reader, Writer};
use crate::net::handler::Pipeline;

use crate::net::handler::MessageRegistry;
use crate::net::messages::{ping::PingMsg, chat::ChatMsg};

pub struct Client {
    reader: Reader,
    pub writer: Writer,
	// pipeline: Pipeline
	handlers: Arc<MessageRegistry>
}
impl Client {
    pub fn new(socket: TcpStream, handlers: Arc<MessageRegistry>) -> Self {
		let (r, w) = socket.into_split(); 
		let reader = Arc::new(Mutex::new(r)); 
		let writer = Arc::new(Mutex::new(w)); 
        Self { reader, writer, handlers } //pipeline }
    }
	pub async fn new_connection(addr: &str, handlers: Arc<MessageRegistry>) -> Result<Self, Box<dyn Error>> {
		let socket = TcpStream::connect(addr).await?;
		Ok(Self::new(socket, handlers))
	}

    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
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
			let msg = self.handlers.deserialize(&buf[0..n]);
			msg.handle(&self).await;

            // let st = std::str::from_utf8(&buf).unwrap();
            // println!("received: {}", st);
            // self.writer.lock().await.write_all(b"ping").await.expect("msg");
        }
		Ok(())
    }
}

