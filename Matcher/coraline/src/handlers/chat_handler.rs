use std::{any::Any, error::Error, sync::Arc};

use async_trait::async_trait;
use teal::net::{
    client::Client, handler::Handler, message::MessageScript, messages::chat::ChatMsg, Message,
};

pub(crate) struct ChatHandler;

#[async_trait]
impl Handler for ChatHandler {
    async fn handle(&self, msg: Message, client: &Client) -> Result<(), Box<dyn Error>> {
        let opt = msg.as_any().downcast_ref::<ChatMsg>();
        if let Some(chat) = opt {
            println!("yo client got chat data {:?}", chat);
            // return client.send(ChatMsg::new()).await;
        }
        Ok(())
    }
}
