use coral::IMessage;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::{
    any::{self, type_name, Any}, collections::HashMap, ptr::null, str::Bytes
};

trait Simplus {}
trait Message {
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
    fn create() -> Box<dyn MessageScript>;
}
pub trait MessageScript {
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
    fn id(&self) -> u8;
    fn handle(&self, client: &dyn Any);
    fn send(&self, socket_maybe: &dyn Any);
    fn serialize(&self) -> Vec<u8>;
}
// pub trait MessageDeserializer {
//     fn deserialize(&self, bytes: &[u8]) -> Box<dyn MessageScript>;
// }

/*
// in the lib
pub struct Flag {
    short: char,
    name: &'static str,
}
impl Flag {
    pub const fn new(short: char, name: &'static str) -> Self {
        Flag { short, name }
    }
}
inventory::collect!(Flag);
fn inventory_stuff() {
    for flag in inventory::iter::<Flag> {
        println!("-{}, --{}", flag.short, flag.name);
    }
}
// in your plugin
inventory::submit! {
    Flag::new('v', "verbose")
}
*/

macro_rules! my_test_macro {
    ($t1:ty) => {};
}
my_test_macro!(Simplus);

// inventory is just not gonna work
// inventory::collect!(ChatMsg);
// inventory::submit! {
// 	ChatMsg { text: String::from(""), channel: String::from("") }
// }

// #[derive(Send)]
pub struct MessageRegistry {
    map: HashMap<u8, fn(&[u8]) -> Box<dyn MessageScript>>,
}
impl MessageRegistry {
    pub fn register(&mut self, id: u8, msg: fn(&[u8]) -> Box<dyn MessageScript>) {
        self.map.insert(id, msg);
    }
    pub fn deserialize(&self, frame: &[u8]) -> Box<dyn MessageScript> {
		let id = frame[0];
		let deserializer = self.map.get(&id).unwrap();
		let dsa = deserializer(&frame[1..]);
		return dsa;
	}
}

fn hello_main() {
    let mut reg = MessageRegistry {
        map: HashMap::new(),
    };
    reg.register(PingMsg::uid(), PingMsg::deserialize);
    reg.register(ChatMsg::uid(), ChatMsg::deserialize);


    let implementations: Vec<Box<dyn MessageScript>> = vec![
        Box::new(PingMsg {
            ..Default::default()
        }),
        Box::new(ChatMsg {
            ..Default::default()
        }),
    ];
    for imp in implementations {
        println!("{}", imp.name());
        let byte_buf: Vec<u8>;
        // serde_bytes::serialize(bytes, serializer)
        // serde_bytes::serialize(&byte_buf, *imp);

        // let encoded: Vec<u8> = bincode::serialize(&imp).unwrap();
        let encoded = imp.serialize();
		
		let msg = reg.deserialize(&encoded[..]);
		// msg.handle({});
    }
}


// derive IMessage
#[derive(Debug, Default, Deserialize, Serialize, new)]
struct PingMsg {
    #[new(default)]
    value: i32,
}
impl PingMsg {
	fn uid() -> u8 { 1 }
    fn deserialize(bytes: &[u8]) -> Box<dyn MessageScript> {
		let i = bincode::deserialize::<Self>(&bytes[..]).unwrap();
		return Box::new(i);
	}
}
impl Message for PingMsg {
    fn create() -> Box<dyn MessageScript> {
        Box::new(PingMsg {
            ..Default::default()
        })
    }
}
// impl MessageDeserializer for PingMsg {
//     fn deserialize(&self, bytes: &[u8]) -> Box<dyn MessageScript> {
// 		let i: PingMsg = bincode::deserialize(&bytes[..]).unwrap();
// 		return Box::new(i);
// 	}
// }
// impl Message for PingMsg;
impl MessageScript for PingMsg {
    fn id(&self) -> u8 { Self::uid() }
    fn handle(&self, client: &dyn Any) {
        println!("yo we got ping data {:?}", self);
    }
    fn send(&self, socket_maybe: &dyn Any) {
        todo!()
    }
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

// derive IMessage
#[derive(Debug, Default, Deserialize, Serialize, new)]
struct ChatMsg {
    #[new(default)]
    channel: String,
    #[new(default)]
    text: String,
}
impl ChatMsg {
	fn uid() -> u8 { 2 }
    fn deserialize(bytes: &[u8]) -> Box<dyn MessageScript> {
		let i: Self = bincode::deserialize(&bytes[..]).unwrap();
		return Box::new(i);
	}
}
impl Message for ChatMsg {
    fn create() -> Box<dyn MessageScript> {
        Box::new(ChatMsg {
            ..Default::default()
        })
    }
}
// impl MessageDeserializer for ChatMsg {
//     fn deserialize(&self, bytes: &[u8]) -> Box<dyn MessageScript> {
// 		let i: ChatMsg = bincode::deserialize(&bytes[..]).unwrap();
// 		return Box::new(i);
// 	}
// }
impl MessageScript for ChatMsg {
    fn id(&self) -> u8 { Self::uid() }
    fn handle(&self, client: &Client) {
        println!("yo we got ping data {:?}", self);
    }
    fn send(&self, socket_maybe: &Client) {
        todo!()
    }
    fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
}

// msg_macro!(
// 	{
// 		channel: String,
// 		text: String,
// 	},
// 	{
// 		println!("yo we got ping data {:?}", self);
// 	},
// 	{
// 		todo!()
// 	}
// );
