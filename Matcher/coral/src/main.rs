//! A "hello world" echo server with Tokio
//!
//! This server will create a TCP listener, accept connections in a loop, and
//! write back everything that's read off of each TCP connection.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! To see this server in action, you can run this in one terminal:
//!
//!     cargo run --example echo
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect 127.0.0.1:8080
//!
//! Each line you type in to the `connect` terminal should be echo'd back to
//! you! If you open up multiple terminals running the `connect` example you
//! should be able to see them all make progress simultaneously.

#![warn(rust_2018_idioms)]
mod handlers;
use handlers::ping_handler::PingHandler;
use handlers::chat_handler::ChatHandler;
use handlers::pong_handler::PongHandler;

use teal::net::handlers::MessageHandlers;
use teal::net::messages::chat::ChatMsg;
use teal::net::messages::ping::PingMsg;
use teal::net::messages::pong::PongMsg;
use teal::net::server::Server;
use std::sync::Arc;

use std::env;
use std::error::Error;
use teal::onyx::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let u: User;
    let u2: teal::onyx::fire::User;
    teal::add(1, 2);

    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let mut reg = MessageHandlers::new();
    reg.register(ChatMsg::uid(), ChatMsg::deserialize, Arc::new(ChatHandler));
    reg.register(PingMsg::uid(), PingMsg::deserialize, Arc::new(PingHandler));
    reg.register(PongMsg::uid(), PongMsg::deserialize, Arc::new(PongHandler));

    Server::run(addr, Arc::new(reg)).await.ok();
    
    Ok(())
}
