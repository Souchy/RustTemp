use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{Reader, Writer};
use crate::net::handler::Pipeline;

use super::handler::MessageRegistry;
use super::messages::ping::PingMsg;

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

			let mut reg = MessageRegistry::new();
			reg.register(PingMsg::uid(), PingMsg::deserialize);
			let msg = reg.deserialize(&buf[0..n]);
			msg.handle(&self);
			// reg.register(ChatMsg::uid(), ChatMsg::deserialize);
            // let st = std::str::from_utf8(&buf).unwrap();
            // println!("received: {}", st);
            // self.writer.lock().await.write_all(b"ping").await.expect("msg");
        }
		Ok(())
    }
}




// use derive_new::new;
// use serde::{Deserialize, Serialize};
// use std::{
//     any::{self, type_name, Any}, collections::HashMap, ptr::null, str::Bytes
// };
// use super::message::MessageScript;

// #[derive(Debug, Default, Serialize, Deserialize)]
// struct ChatMsg {
//     channel: String,
//     text: String,
// }
// impl ChatMsg {
// 	fn uid() -> u8 { 2 }
//     fn deserialize(bytes: &[u8]) -> Box<dyn MessageScript> {
// 		let i: Self = bincode::deserialize(&bytes[..]).unwrap();
// 		return Box::new(i);
// 	}
// }
// // impl Message for ChatMsg {
// //     fn create() -> Box<dyn MessageScript> {
// //         Box::new(ChatMsg {
// //             ..Default::default()
// //         })
// //     }
// // }
// impl MessageScript for ChatMsg {
//     fn id(&self) -> u8 { Self::uid() }
//     async fn handle(&self, client: &Client) {
//         println!("yo we got ping data {:?}", self);
// 		client.writer.lock().await.write_all(b"pong").await.expect("msg");
//     }
//     fn send(&self, socket_maybe: &Client) {
//         todo!()
//     }
//     fn serialize(&self) -> Vec<u8> {
//         bincode::serialize(&self).unwrap()
//     }
// }
