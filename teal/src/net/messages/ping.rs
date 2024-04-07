use async_trait::async_trait;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::{
    any::{self, type_name, Any}, collections::HashMap, error::Error, ptr::null, str::Bytes, sync::Arc
};
use tokio::io::AsyncWriteExt;

use crate::net::{client::Client, message::MessageScript};


// derive IMessage
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PingMsg {
    value: i32,
}
impl PingMsg {
    pub fn uid() -> u8 {
        1
    }
    pub fn deserialize(bytes: &[u8]) -> Arc<dyn MessageScript + Sync + Send> {
        let i = bincode::deserialize::<Self>(&bytes[..]).unwrap();
        return Arc::new(i);
    }
}

#[async_trait]
impl MessageScript for PingMsg {
    fn id(&self) -> u8 {
        Self::uid()
    }
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("yo we got ping data {:?}", self);
        client.send(b"pong").await
        // client
        //     .writer
        //     .lock()
        //     .await
        //     .write_all(b"pong")
        //     .await
        //     .expect("msg");
		// Ok(())
    }
    async fn send(&self, socket_maybe: &Client) -> Result<(), Box<dyn Error>> {
        socket_maybe.send(&MessageScript::serialize(self)).await
    }
}
