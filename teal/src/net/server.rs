use std::{error::Error, sync::Arc};

use tokio::sync::broadcast;
use tokio::{net::TcpListener, sync::Mutex};
use tracing::{debug, error, info, instrument};

use crate::net::client::Client;
use crate::net::handlers::MessageHandlers;

use super::Message;

pub struct Server {
    clients: Vec<Arc<Client>>,
}

impl Server {

    pub async fn run(addr: String, handlers: Arc<MessageHandlers>) -> Result<(), Box<dyn Error>> {
        // Next up we create a TCP listener which will listen for incoming
        // connections. This TCP listener is bound to the address we determined
        // above and must be associated with an event loop.
        let listener = TcpListener::bind(&addr).await?;
        println!("Listening on: {}", addr);

        let server = Self {
            clients: Vec::new(),
        };
        let server_ptr = Arc::new(Mutex::new(server));

        // let (tx, rx) = broadcast::channel::<&[u8]>(1000);
        // tokio::select! {
            
        // }

        loop {
            // Asynchronously wait for an inbound socket.
            let (socket, _addr) = listener.accept().await?;

            // And this is where much of the magic of this server happens. We
            // crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the
            // `tokio::spawn` function to execute the work in the background.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to be processed concurrently.
            // let server_ref = Arc::new(self);

            let client: Client = Client::new(socket, handlers.clone(), Some(server_ptr.clone()));

            let client_runner = Arc::new(client);
            server_ptr.lock().await.clients.push(client_runner.clone());

            tokio::spawn(async move {
                // client.run().await ;
                if let Err(err) = client_runner.run().await {
                    error!(cause = ?err, "client connection error");
                }
            });
        }
    }

    pub async fn broadcast(&self, msg: Message) {
		for c in &self.clients {
			c.send(msg.clone()).await.ok();
		}
	}
}
