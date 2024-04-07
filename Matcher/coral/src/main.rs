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

use bytes::{Buf, BytesMut};
use teal::net::handler::MessageRegistry;
use teal::net::messages::chat::ChatMsg;
use teal::net::messages::ping::PingMsg;
use teal::net::server::Server;
use std::io::{self, Cursor};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, info, instrument};

use miniredis::server;
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

    let mut reg = MessageRegistry::new();
    reg.register(PingMsg::uid(), PingMsg::deserialize);
    reg.register(ChatMsg::uid(), ChatMsg::deserialize);

    Server::new(Arc::new(reg)).run(addr).await

    /*
    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    // crate::red::red::run(listener);

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, _) = listener.accept().await?;

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.

        let mut client: Client = Client::new(socket);

        tokio::spawn(async move {
            if let Err(err) = client.run().await {
                error!(cause = ?err, "connection error");
            }

            // let mut buf = vec![0; 1024];

            // // In a loop, read data from the socket and write the data back.
            // loop {
            //     let n = socket
            //         .read(&mut buf)
            //         .await
            //         .expect("failed to read data from socket");

            //     if n == 0 {
            //         return;
            //     }

            //     let st = std::str::from_utf8(&buf).unwrap();
            //     println!("received: {}", st);

            //     socket
            //         .write_all(&buf[0..n])
            //         .await
            //         .expect("failed to write data to socket");
            // }
        });
    }
    */
}

// pub type Result<T> = std::result::Result<T, Error>;

/*
#[derive(Debug)]
pub struct Client {
    socket: TcpStream,
    // stream: BufWriter<TcpStream>,
    // buffer: BytesMut,
}
impl Client {
    pub fn new(socket: TcpStream) -> Client {
        Client {
            socket,
            // stream: BufWriter::new(socket),
            // buffer: BytesMut::with_capacity(4 * 1024),
        }
    }
    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut buf = vec![0; 4 * 1024];
        // In a loop, read data from the socket and write the data back.
        // loop {
        //     let n = self.socket
        //         .read(&mut buf)
        //         .await
        //         .expect("failed to read data from socket");

        //     if n == 0 {
        //         return;
        //     }

        //     let st = std::str::from_utf8(&buf).unwrap();
        //     println!("received: {}", st);

        //     self.socket
        //         .write_all(&buf[0..n])
        //         .await
        //         .expect("failed to write data to socket");
        // }

        // with buffer
        loop {
            // let n = self.stream.read_buf(&mut self.buffer).await?;
            let n = self
                .socket
                .read(&mut buf)
                .await
                .expect("failed to read data from socket");
            if n == 0 {
                println!("client connected terminated");
                return Ok(());
            }
            // let mut buf = Cursor::new(&self.buffer[..]);
            // let len = buf.read_u16();
            // let cl = self.buffer.clone();

            // let st = std::str::from_utf8(&self.buffer).unwrap();
            let st = std::str::from_utf8(&buf[0..n]).unwrap();

            // self.stream
            //     .write_all(b"pong")
            //     .await
            //     .expect("failed to write data to socket");
            self.socket
                .write_all(b"pong")
                .await
                .expect("failed to write data to socket");

            println!("received: {}", st);

            // self.buffer.advance(n);
        }
        Ok(())
    }
}
*/
/*
struct Listener;
impl Listener {
    pub async fn run(&mut self) -> crate::Result<()> {
        info!("accepting inbound connections");

        loop {
            // Wait for a permit to become available
            //
            // `acquire_owned` returns a permit that is bound to the semaphore.
            // When the permit value is dropped, it is automatically returned
            // to the semaphore.
            //
            // `acquire_owned()` returns `Err` when the semaphore has been
            // closed. We don't ever close the semaphore, so `unwrap()` is safe.
            let permit = self
                .limit_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();

            // Accept a new socket. This will attempt to perform error handling.
            // The `accept` method internally attempts to recover errors, so an
            // error here is non-recoverable.
            let socket = self.accept().await?;

            // Create the necessary per-connection handler state.
            let mut handler = Handler {
                // Get a handle to the shared database.
                // db: self.db_holder.db(),

                // Initialize the connection state. This allocates read/write
                // buffers to perform redis protocol frame parsing.
                connection: Connection::new(socket),

                // Receive shutdown notifications.
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),

                // Notifies the receiver half once all clones are
                // dropped.
                _shutdown_complete: self.shutdown_complete_tx.clone(),
            };

            // Spawn a new task to process the connections. Tokio tasks are like
            // asynchronous green threads and are executed concurrently.
            tokio::spawn(async move {
                // Process the connection. If an error is encountered, log it.
                if let Err(err) = handler.run().await {
                    error!(cause = ?err, "connection error");
                }
                // Move the permit into the task and drop it after completion.
                // This returns the permit back to the semaphore.
                drop(permit);
            });
        }
    }
}
*/
