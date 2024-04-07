use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

use crate::net::{client::Client, message::MessageScript};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PongMsg {}
impl PongMsg {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
    pub fn uid() -> u8 {
        2
    }
    pub fn deserialize(bytes: &[u8]) -> Arc<dyn MessageScript + Sync + Send> {
        let i = bincode::deserialize::<Self>(&bytes[..]).unwrap();
        return Arc::new(i);
    }
}

#[async_trait]
impl MessageScript for PongMsg {
    fn id(&self) -> u8 {
        Self::uid()
    }
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>> {
        println!("yo we got pong data {:?}", self);
        Ok(())
    }
}
