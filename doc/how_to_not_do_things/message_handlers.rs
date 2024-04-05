
/*
 * 
 * So,
 * Generic trait causes a lot of problems when collecting them into a list/map
 * Same with using Box<dyn Trait> shit
 * 
 * We can just apply the struct + script pattern again. 
 * 
 * struct MyMsg {
 * 		value: i32;
 * }
 * impl MsgScript for MyMsg {
 * 		fn script() {
 * 			// can access value because we're already in MyMsg object
 * 		}
 * }
 * 
 * 
 * This is also similar to extension methodes
 * 
 */


use std::{any::{self, type_name, Any}, collections::HashMap};

use coral::IMessage;


trait Message { }
// trait IHandler {
// 	fn handle_message(&self, msg: Message);
// }

// Define a trait object wrapper for the generic trait
// trait AnyMessageHandler {
//     fn handle_message(&self, msg : Box<dyn Any>);
// }
trait MessageHandlerGeneric<T> {
	fn name(&self) -> String {
		String::from(type_name::<Self>())
	}
	fn handle_message(&self, msg: T);
}
trait MessageHandler {
	fn name(&self) -> String {
		String::from(type_name::<Self>())
	}
	fn handle_message(&self, msg: Box<dyn Message>);
}
trait MessageHandlesItself {
	fn name(&self) -> String {
		String::from(type_name::<Self>())
	}
	fn handle(&self, client: &dyn Any);
}
// Implement the trait object wrapper for the generic trait
// impl<T> AnyMessageHandler for dyn MessageHandler<T> {
//     fn handle_message(&self) {
//         // Placeholder function; the actual implementation will depend on the concrete type
//         println!("Message handling for some type");
//     }
// }



fn hello() {
	
    // Create instances of handlers
    let ping_handler = PingHandler;
    let chat_handler = ChatMsgHandler;

    // Create a HashMap containing trait objects
    let mut handlers: HashMap<&str, MessageHandler> = HashMap::new();

    // Insert concrete handlers into the HashMap
    handlers.insert("ping", ping_handler);
    handlers.insert("chat", chat_handler);


    // Iterate over handlers and call name function for each
    for handler in &handlers {
        println!("Handler name: {}", handler.name());
    }

    // let mut map = HashMap::new();

	for flag in inventory::iter::<Box<dyn MessageHandler>> {
		println!("-{}, --{}", flag.short, flag.name);
	}
}



#[derive(Debug, IMessage)]
struct PingMsg {
	value: i32,
}
#[derive(Debug)]
struct PingHandler;
impl MessageHandler for PingHandler {
	fn handle_message(&self, msg: Box<dyn Message>) {
		let ping: PingMsg = *msg;
		// do something like println
        println!("Doing something with {:?}", ping);
	}
}
impl MessageHandlesItself for PingMsg {
	fn handle(&self, client: &dyn Any) {
		println!("yo we got ping data {:?}", self);
	}
}


#[derive(Debug, IMessage)]
struct ChatMsg;
#[derive(Debug)]
struct ChatMsgHandler;
// impl MessageHandler<ChatMsg> for ChatMsgHandler {
// 	fn handle_message(&self, msg: ChatMsg) {
// 		// do something different like write to disk
// 	}
// }

