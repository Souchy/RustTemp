
use miniredis::server;

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{self, Duration};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let addr = listener.local_addr().unwrap();

    println!("Server up! 127.0.0.1:8080");
    // tokio::spawn(async move {
        server::run(listener, tokio::signal::ctrl_c()).await 
    // });

}
