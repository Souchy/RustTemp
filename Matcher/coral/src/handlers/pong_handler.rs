use std::error::Error;

use async_trait::async_trait;
use teal::net::messages::ping::PingMsg;
use teal::net::messages::pong::PongMsg;
use teal::net::{client::Client, handler::Handler, Message};

pub(crate) struct PongHandler;

#[async_trait]
impl Handler for PongHandler {
    async fn handle(&self, msg: Message, client: &Client) -> Result<(), Box<dyn Error>> {
        let opt = msg.as_any().downcast_ref::<PongMsg>();
        if let Some(m) = opt {
            println!("yo server got pong data {:?}", m);
        }
        Ok(())
    }
}
