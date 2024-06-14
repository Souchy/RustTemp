use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{any::Any, error::Error, sync::Arc};

use crate::net::{client::Client, message::MessageScript, messages::pong::PongMsg, Message};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PingMsg {}
impl PingMsg {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
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
        client.send(PongMsg::new()).await
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
