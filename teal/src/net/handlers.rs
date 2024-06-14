use std::{collections::HashMap, error::Error, rc::Rc};

use futures::Future;

use super::{client::Client, handler::Handler, messages::ping::PingMsg, Message, MsgHandler};

pub struct MessageHandlers {
    deserializers: HashMap<u8, fn(&[u8]) -> Message>,
    handlers: HashMap<u8, MsgHandler>,
    pub invalid_message_handler: fn(&[u8], &Client),
}

impl MessageHandlers {
    pub fn new() -> Self {
        Self {
            deserializers: HashMap::new(),
            handlers: HashMap::new(),
            invalid_message_handler: Self::default_handler,
        }
    }
    pub fn register(&mut self, id: u8, msg: fn(&[u8]) -> Message, handler: MsgHandler) {
        self.deserializers.insert(id, msg);
        self.handlers.insert(id, handler);
    }

    fn deserialize(&self, frame: &[u8]) -> Option<Message> {
        let len = frame[0] as usize;
        let id = frame[1];
        let deserializer = self.deserializers.get(&id)?;
        let script = deserializer(&frame[2..len]);

        return Some(script);
    }

    pub async fn handle(&self, frame: &[u8], client: &Client) -> Result<(), Box<dyn Error>> {
        match self.deserialize(&frame) {
            Some(msg) => {
                if let Some(handler) = self.handlers.get(&msg.id()) {
                    return handler.handle(msg, client).await;
                }
                Ok(())
            }
            None => {
                (self.invalid_message_handler)(&frame, &client);
                Ok(())
            }
        }
    }

    fn default_handler(frame: &[u8], client: &Client) {
        let st = std::str::from_utf8(&frame).unwrap();
        println!("received invalid message: {}", st);
    }
}
