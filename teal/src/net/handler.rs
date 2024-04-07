// struct Handler;
// impl Handler {
// 	fn handle(buf: &[u8]) {
		
// 	}
// }

// trait Handler {
// 	fn handle(buf: &[u8]);
// 	fn then(handler: Box<dyn Handler>);
// }

// struct Handler {
// 	action: fn(),
// 	pipeline: Vec<Box<dyn Handler>>,
// }
// impl Handler {
// 	fn new(action: fn()) -> Self {
// 		Self {
// 			action,
// 			pipeline: vec![]
// 		}
// 	}
// 	fn then(&self, Box) {
// 		self.pipeline.push()
// 	}
// }

#[derive(Clone)]
pub struct Pipeline {
	handlers: Vec<fn()>
}
impl Pipeline {
	pub fn handle(&self, buf: &[u8]) {
		for h in &self.handlers {
			h()
		}
	}
}

// struct Handler<I, O> {
// 	fun: fn(I) -> O
// }
// impl<I, O> Handler<I, O> {
// 	fn handle(&self, i: I) -> O {
// 		return self.fun(i);
// 	}
// }

// Handlers:
//	1. packet parser fn(&[u8]) -> Message
//	2. message handler
use serde::{Deserialize, Serialize};
use std::{
    any::{self, type_name, Any}, collections::HashMap, ptr::null, str::Bytes, sync::Arc
};
use crate::net::message::MessageScript;

pub struct MessageRegistry {
    pub map: HashMap<u8, fn(&[u8]) -> Arc<dyn MessageScript>>,
}
impl MessageRegistry {
	pub fn new() -> Self {
		Self {
			map: HashMap::new(),
		}
	}
    pub fn register(&mut self, id: u8, msg: fn(&[u8]) -> Arc<dyn MessageScript>) {
        self.map.insert(id, msg);
    }
    pub fn deserialize(&self, frame: &[u8]) -> Arc<dyn MessageScript> {
		let id = frame[0];
		let deserializer = self.map.get(&id).unwrap();
		let dsa = deserializer(&frame[1..]);
		return dsa;
	}
}
