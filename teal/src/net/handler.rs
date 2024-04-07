use std::{collections::HashMap, error::Error};

use super::{client::Client, Message};

pub struct MessageHandlers {
    map: HashMap<u8, fn(&[u8]) -> Message>,
    pub invalid_message_handler: fn(&[u8], &Client),
}

impl MessageHandlers {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            invalid_message_handler: Self::default_handler,
        }
    }
    pub fn register(&mut self, id: u8, msg: fn(&[u8]) -> Message) {
        self.map.insert(id, msg);
    }

    fn deserialize(&self, frame: &[u8]) -> Option<Message> {
        let id = frame[0];
        let deserializer = self.map.get(&id)?;
        let script = deserializer(&frame[1..]);
        return Some(script);
    }

    pub async fn handle(&self, frame: &[u8], client: &Client) -> Result<(), Box<dyn Error>> {
        match self.deserialize(&frame) {
            Some(msg) => msg.handle(client).await,
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
