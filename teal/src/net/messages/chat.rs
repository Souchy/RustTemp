use crate::net::{client::Client, message::MessageScript};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ChatMsg {
    pub channel: String,
    pub text: String,
}
impl ChatMsg {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
    pub fn uid() -> u8 {
        3
    }
    pub fn deserialize(bytes: &[u8]) -> Arc<dyn MessageScript + Sync + Send> {
        let i: Self = bincode::deserialize(&bytes[..]).unwrap();
        return Arc::new(i);
    }
}

#[async_trait]
impl MessageScript for ChatMsg {
    fn id(&self) -> u8 {
        Self::uid()
    }
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("yo we got chat data {:?}", self);
        Ok(())
    }
}
