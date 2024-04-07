// Handlers:
//	1. packet parser fn(&[u8]) -> Message
//	2. message handler
use std::{collections::HashMap, error::Error, future::Future};

use futures::future::BoxFuture;

use super::{client::Client, Message};

// trait AsyncFn {
//     fn call(&self, args: u8) -> BoxFuture<'static, u8>;
// }
// impl<T, F> AsyncFn for T
// where
//     T: Fn(u8) -> F,
//     F: Future<Output = u8> + 'static,
// {
//     fn call(&self, args: u8) -> BoxFuture<'static, u8> {
//         Box::pin(self(args))
//     }
// }

pub struct MessageHandlers
{
    map: HashMap<u8, fn(&[u8]) -> Message>,
    pub alo: fn(&[u8], &Client)
    // pub default: Box<dyn 
	// 	Fn(&[u8], &Client) -> BoxFuture<'static, Result<(), Box<dyn Error>>>
	// >,
}
impl MessageHandlers {
    pub fn new() -> Self {
		// let f = |frame: &[u8], client: &Client| async {
		// 	let st = std::str::from_utf8(&frame).unwrap();
		// 	println!("received: {}", st);
		// 	client.send_bytes(b"invalid message").await;
		// };
		let f = |frame: &[u8], client: &Client| {
			let st = std::str::from_utf8(&frame).unwrap();
			println!("received invalid message: {}", st);
            // async move {
            //     client.send_bytes(b"invalid message").await;
            // };
		};
        Self {
            map: HashMap::new(),
            // default: Box::new(Self::foo),
            alo: f
        }
    }

	async fn foo(frame: &[u8], client: &Client) -> Result<(), Box<dyn Error>> {
        Ok(())
	}
    
    pub fn register(&mut self, id: u8, msg: fn(&[u8]) -> Message) {
        self.map.insert(id, msg);
    }
    pub fn deserialize(&self, frame: &[u8]) -> Option<Message> {
        let id = frame[0];
        let deserializer = self.map.get(&id)?;
        let script = deserializer(&frame[1..]);
        return Some(script);
    }
    pub async fn handle(&self, frame: &[u8], client: &Client) -> Result<(), Box<dyn Error>> {
        match self.deserialize(&frame) {
            Some(msg) => msg.handle(client).await,
            None => {
                (self.alo)(&frame, &client);
                Ok(()) //Ok((self.default)(&frame, client).await),
            }
        }
    }
}
