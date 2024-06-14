use std::{any::Any, pin::Pin, sync::Arc};
use client::Client;
use futures::Future;
use handler::Handler;

use self::message::MessageScript;

pub mod client;
pub mod server;
pub mod handler;
pub mod handlers;
pub mod message;
pub mod messages;

pub type Message = Arc<dyn MessageScript + Send + Sync>;
pub type MsgHandler = Arc<dyn Handler + Send + Sync>;
// pub type MsgHandler = Box<dyn FnOnce(Message, &Client) -> Pin<Box<dyn Future<Output = ()>>>>;
