use std::{
    any::{self, type_name, Any},
    collections::HashMap,
};

use coral::IMessage;

trait Simplus {}
trait Message {
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
}
trait MessageScript {
    fn name(&self) -> String {
        String::from(type_name::<Self>())
    }
	fn id(&self) -> i32;
    fn handle(&self, client: &dyn Any);
    fn send(&self, socket_maybe: &dyn Any);
}


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

inventory::collect!(ChatMsg);
inventory::submit! {
	ChatMsg { ..Default::default() }
}
struct PluginRegistry {
	map: HashMap<i32, Box<dyn MessageScript>>,
}


fn register(msg: Box<dyn MessageScript>) {
	let mut reg = PluginRegistry { map: HashMap::new() };
	reg.map.insert(msg.id(), msg);
}

fn hello() {
    let implementations: Vec<Box<dyn MessageScript>> = vec![
		Box::new(PingMsg { ..Default::default() }),
		Box::new(ChatMsg { ..Default::default() }),
	];
	for imp in implementations {
		println!("{}", imp.name());
	}
}

#[derive(IMessage, Debug, Default)]
struct PingMsg {
    value: i32,
}
// impl Message for PingMsg;
impl MessageScript for PingMsg {
	fn id(&self) -> i32 { 1 }
    fn handle(&self, client: &dyn Any) {
        println!("yo we got ping data {:?}", self);
    }
    fn send(&self, socket_maybe: &dyn Any) {
        todo!()
    }
}

#[derive(IMessage, Debug, Default)]
struct ChatMsg {
    channel: String,
    text: String,
}
impl MessageScript for ChatMsg {
	fn id(&self) -> i32 { 2 }
    fn handle(&self, client: &dyn Any) {
        println!("yo we got ping data {:?}", self);
    }
    fn send(&self, socket_maybe: &dyn Any) {
        todo!()
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
