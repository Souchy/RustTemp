
use serde::{Deserialize, Serialize};
use std::{
    any::{self, type_name, Any}, collections::HashMap, ptr::null, str::Bytes
};
use super::client::Client;


pub trait MessageScript {
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
    fn id(&self) -> u8;
    fn serialize(&self) -> Vec<u8>;
    async fn handle(&self, client: &Client);
    async fn send(&self, socket_maybe: &Client);
}
