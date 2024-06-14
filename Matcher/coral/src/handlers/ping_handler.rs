use std::error::Error;

use async_trait::async_trait;
use teal::net::messages::ping::PingMsg;
use teal::net::messages::pong::PongMsg;
use teal::net::{client::Client, handler::Handler, Message};

pub(crate) struct PingHandler;

#[async_trait]
impl Handler for PingHandler {
    async fn handle(&self, msg: Message, client: &Client) -> Result<(), Box<dyn Error>> {
        let opt = msg.as_any().downcast_ref::<PingMsg>();
        if let Some(ping) = opt {
            println!("yo server got ping data {:?}", ping);
            return client.send(PongMsg::new()).await;
        }
        Ok(())
    }
}
