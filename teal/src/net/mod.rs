use std::sync::Arc;
use self::message::MessageScript;


pub mod client;
pub mod server;
pub mod handler;
pub mod message;
pub mod messages;

type Message = Arc<dyn MessageScript + Send + Sync>;
