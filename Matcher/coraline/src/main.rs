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

use teal::net::client::Client;
use teal::net::handler::MessageRegistry;
use teal::net::messages::chat::ChatMsg;
use teal::net::messages::ping::PingMsg;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf, ReadHalf, WriteHalf};
use tokio::net::TcpStream;

use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

type Reader = Arc<Mutex<OwnedReadHalf>>;
type Writer = Arc<Mutex<OwnedWriteHalf>>;

// type Reader1 = Arc<Mutex<ReadHalf<TcpStream>>>;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    /*/
    let s = TcpStream::connect("127.0.0.1:8080").await?;
    let (mut r, mut w) = s.into_split(); //tokio::io::split(s);
                                         // let reader = Arc::new(r);
                                         // let writer = Arc::new(w); //Arc::clone(&reader);

    // let stream = Arc::new(Mutex::new(s));
    let reader = Arc::new(Mutex::new(r)); //stream.clone(); //Arc::clone(&stream);
    let writer = Arc::new(Mutex::new(w)); //Arc::clone(&stream);
    let writer2 = writer.clone();
    // let dsa: OwnedWriteHalf;

    println!("created stream");

    let client = Client::new(reader, writer);
    */
    let mut reg = MessageRegistry::new();
    reg.register(PingMsg::uid(), PingMsg::deserialize);
    reg.register(ChatMsg::uid(), ChatMsg::deserialize);
    let client: Client = Client::new_connection("127.0.0.1:8080", Arc::new(reg)).await?;

    let client_ref = Arc::new(client);
    let client_ref2 = client_ref.clone();

    let t1 = tokio::spawn(async move {
        client_ref.run().await;
    });
    let t2 = tokio::spawn(async move {
        println!("t2 start");
        /*
        // let mut tw = writer.lock().await;
        let mut result = writer2
            .lock()
            .await
            .write_all(b"hello world1")
            .await;
        */
        let result = client_ref2.send(b"hello world1").await;
        println!("wrote to stream; success={:?}", result.is_ok());

        // let res2 = writer2
        //     .lock()
        //     .await
        //     // .lock().unwrap()
        //     .write_all(b"hello world2")
        //     .await;
        // println!("wrote to stream; success={:?}", res2.is_ok());
    });

    t2.await?;
    let out = t1.await.unwrap();

    // tokio::join!(t1);

    Ok(())
}

/*
struct Client {
    reader: Reader,
    writer: Writer,
}
impl Client {
    fn new(reader: Reader, writer: Writer) -> Self {
        Self { reader, writer }
    }
    async fn run(&self) {
        println!("t1 start");
        let mut buf = vec![0; 4 * 1024];
        loop {
            let mut tr = self.reader.lock().await;
            // let mut tr = r;
            let n = tr
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
            self.writer.lock().await.write_all(b"ping").await.expect("msg");
        }
    }
}
*/
