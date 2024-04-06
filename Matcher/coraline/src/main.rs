//! A simple client that opens a TCP stream, writes "hello world\n", and closes
//! the connection.
//!
//! To start a server that this client can talk to on port 6142, you can use this command:
//!
//!     ncat -l 6142
//!
//! And then in another terminal run:
//!
//!     cargo run --example hello_world

#![warn(rust_2018_idioms)]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use std::error::Error;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("created stream");

    let mut result = stream.write_all(b"hello world1\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());
    
    result = stream.write_all(b"hello world2\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());

    loop {
        let mut buf = vec![0; 1024];
        let n = stream
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            println!("client connected terminated");
            break;
        }
        
        let st = std::str::from_utf8(&buf).unwrap();
        println!("received: {}", st);
    }

    Ok(())
}
