use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{Reader, Writer};
use crate::net::handler::Pipeline;

use crate::net::handler::MessageRegistry;
use crate::net::messages::{ping::PingMsg, chat::ChatMsg};

pub struct Client {
    reader: Reader,
    pub writer: Writer,
	pipeline: Pipeline
}
impl Client {
    pub fn new(reader: Reader, writer: Writer, pipeline: Pipeline) -> Self {
        Self { reader, writer, pipeline }
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
			self.pipeline.handle(&buf);

			// let mut reg = MessageRegistry::new();
			// reg.register(PingMsg::uid(), PingMsg::deserialize);
			// reg.register(ChatMsg::uid(), ChatMsg::deserialize);
			// let msg = reg.deserialize(&buf[0..n]);
			// let res = msg.handle(&self).await?;


            // let st = std::str::from_utf8(&buf).unwrap();
            // println!("received: {}", st);
            // self.writer.lock().await.write_all(b"ping").await.expect("msg");
        }
		Ok(())
    }
}

