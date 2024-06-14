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

use super::Message;

// #[async_trait]
// pub trait MessageHandler {
	
//     fn id(&self) -> u8;
	
//     async fn handle(&self, client: &Client) -> Result<(), Box<dyn Error>>;
// }


#[async_trait]
pub trait Handler {
    async fn handle(&self, msg: Message, client: &Client) -> Result<(), Box<dyn Error>>;
}
