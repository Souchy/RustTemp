use crate::net::client::Client;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    any::{self, type_name, Any},
    collections::HashMap,
    error::Error,
    ptr::null,
    str::Bytes,
    sync::Arc,
};

#[async_trait]
pub trait MessageScript {
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
    fn id(&self) -> u8;
    fn serialize(&self) -> Vec<u8>;
    async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>>;
    async fn send(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        let mut buf = MessageScript::serialize(self);
        buf.insert(0, buf.len() as u8 + 2);
        buf.insert(1, self.id());
        client.send_bytes(&buf).await
    }
}




// #[derive(Debug, Default, Deserialize, Serialize)]
// pub struct Macro {
// }
// impl Macro {
// }
// impl Message for Macro {
//     fn id(&self) -> u8 { 1 }
// }
// #[async_trait]
// pub trait MessageReader { // Sized
//     async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>>;
//     // fn deserialize(bytes: &[u8]) -> Arc<dyn MessageReader + Sync + Send>;
// }
// #[async_trait]
// pub trait MessageWriter: Message + Serialize {
//     // fn serialize(&self) -> Vec<u8>;
//     fn serialize(&self) -> Vec<u8> {
//         bincode::serialize(&self).unwrap()
//     }
//     async fn send(&self, client: &Client) -> Result<(), Box<dyn Error>> {
//         let mut buf = MessageWriter::serialize(self);
//         buf.insert(0, buf.len() as u8 + 2);
//         buf.insert(1, self.id());
//         client.send_bytes(&buf).await
//     }
// }


// trait Message {}
// trait MessageHandler<T: Message>  {
//     fn handle(msg: T);
// }

// struct ChatMsg {
//     text: String
// }
// impl Message for ChatMsg {}

// struct ChatMsgHandler;
// impl<T> MessageHandler<T> for ChatMsgHandler where T : ChatMsg {
//     fn handle(msg: ChatMsg) {
//         println!("received chat message {:?}", msg.text);
//     }
// }


// #[derive(Debug, Default, Deserialize, Serialize)]
// struct Broad {
    
// }

// impl Broad {
// }
// impl Message for Broad {
//     // fn id(&self) -> u8 {
//     //     1
//     // }
// }
// #[derive(Debug, Default, Deserialize, Serialize)]
// struct BroadHandler;
// #[async_trait]
// impl MessageReader for BroadHandler {
//     // fn deserialize(bytes: &[u8]) -> Arc<dyn MessageReader + Sync + Send> {
//     //     let i = bincode::deserialize::<Self>(&bytes[..]).unwrap();
//     //     return Arc::new(i);
//     // }
//     async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>> {
//         println!("yo we got pong data {:?}", self);
//         Ok(())
//     }
// }
// impl MessageWriter for Broad {}
