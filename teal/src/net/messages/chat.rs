

use async_trait::async_trait;
use derive_new::new;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use std::{
    any::{self, type_name, Any}, collections::HashMap, error::Error, ptr::null, str::Bytes
};
use crate::net::{client::Client, message::MessageScript};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChatMsg {
    channel: String,
    text: String,
}
impl ChatMsg {
	pub fn uid() -> u8 { 2 }
    pub fn deserialize(bytes: &[u8]) -> Box<dyn MessageScript> {
		let i: Self = bincode::deserialize(&bytes[..]).unwrap();
		return Box::new(i);
	}
}

#[async_trait]
impl MessageScript for ChatMsg {
    fn id(&self) -> u8 { Self::uid() }
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("yo we got ping data {:?}", self);
		client.writer.lock().await.write_all(b"pong").await.expect("msg");
        Ok(())
    }
    async fn send(&self, socket_maybe: &Client) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
