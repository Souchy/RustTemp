
use serde::{Deserialize, Serialize};
use std::{
    any::{self, type_name, Any}, collections::HashMap, error::Error, ptr::null, str::Bytes
};
use crate::net::client::Client;


pub trait MessageScript {
    fn name(&self) -> String {
        String::from("") //type_name::<Self>())
    }
    fn id(&self) -> u8;
    fn serialize(&self) -> Vec<u8>;
    async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>>;
    async fn send(&self, socket_maybe: &Client)  -> Result<(), Box<dyn Error>>;
}
