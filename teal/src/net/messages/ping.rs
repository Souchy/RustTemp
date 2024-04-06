use derive_new::new;
use serde::{Deserialize, Serialize};
use std::{
    any::{self, type_name, Any},
    collections::HashMap,
    ptr::null,
    str::Bytes,
};

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
    pub fn deserialize(bytes: &[u8]) -> Box<dyn MessageScript> {
        let i = bincode::deserialize::<Self>(&bytes[..]).unwrap();
        return Box::new(i);
    }
}

impl MessageScript for PingMsg {
    fn id(&self) -> u8 {
        Self::uid()
    }
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    async fn handle(&self, client: &Client) {
        println!("yo we got ping data {:?}", self);
        client
            .writer
            .lock()
            .await
            .write_all(b"pong")
            .await
            .expect("msg");
    }
    async fn send(&self, socket_maybe: &Client) {
        todo!()
    }
}
