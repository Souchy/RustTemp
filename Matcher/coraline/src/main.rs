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
use std::sync::{Arc, Mutex};


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let s = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut r, mut w) = tokio::io::split(s);
    // let reader = Arc::new(r);
    // let writer = Arc::new(w); //Arc::clone(&reader);

    // let stream = Arc::new(Mutex::new(s));
    // let reader = Arc::clone(&stream);
    // let writer = Arc::clone(&stream);
    
    println!("created stream");

    let t2 = tokio::spawn(async move {
        println!("t2 start");
        // let mut t = writer.lock().unwrap();
        let mut result = w
            .write_all(b"hello world1").await;
        println!("wrote to stream; success={:?}", result.is_ok());
        
        result = w
            // .lock().unwrap()
            .write_all(b"hello world2").await;
        println!("wrote to stream; success={:?}", result.is_ok());
    });
    
    let t1 = tokio::spawn(async move {
        loop {
            // println!("t1 start");
            let mut buf = vec![0; 1024];
            // let mut tr = reader.lock().unwrap();
            // let mut tr = r;
            let n = r
                // .lock().unwrap()
                .read(&mut buf)
                .await
                .expect("failed to read data from socket");
            // println!("read!: {}", n);
            if n == 0 {
                println!("client connected terminated");
                break;
            }
            
            let st = std::str::from_utf8(&buf).unwrap();
            println!("received: {}", st);
        }
    });

    let out = t1.await.unwrap();
        
    // tokio::join!(t1);

    Ok(())
}
