
use std::{error::Error, sync::Arc};

use tokio::{net::TcpListener, sync::Mutex};
use tracing::{debug, error, info, instrument};

use crate::net::client::Client;
use crate::net::handler::MessageHandlers;

use super::Message;


pub struct Server {
	// pipeline: Pipeline,
	handlers: Arc<MessageHandlers>,
	clients: Vec<Arc<Client>>
}

impl Server {
	pub fn new(/* pipeline: Pipeline, */ handlers: Arc<MessageHandlers>) -> Self {
		Self {
			// pipeline,
			handlers,
			clients: Vec::new()
		}
	}
	pub async fn run(&mut self, addr: String) -> Result<(), Box<dyn Error>> {
		// let addr = env::args()
        // .nth(1)
        // .unwrap_or_else(|| "127.0.0.1:8080".to_string());

		// Next up we create a TCP listener which will listen for incoming
		// connections. This TCP listener is bound to the address we determined
		// above and must be associated with an event loop.
		let listener = TcpListener::bind(&addr).await?;
		println!("Listening on: {}", addr);

		
		loop {
			// Asynchronously wait for an inbound socket.
			let (socket, _) = listener.accept().await?;
			
			// And this is where much of the magic of this server happens. We
			// crucially want all clients to make progress concurrently, rather than
			// blocking one on completion of another. To achieve this we use the
			// `tokio::spawn` function to execute the work in the background.
			//
			// Essentially here we're executing a new task to run concurrently,
			// which will allow all of our clients to be processed concurrently.

			let client: Client = Client::new(socket, self.handlers.clone()); //self.pipeline.clone());
			let client_ref = Arc::new(client);
			let client_ref2 = client_ref.clone();
			self.clients.push(client_ref);

			tokio::spawn(async move {
				// client.run().await ;
				if let Err(err) = client_ref2.run().await {
					error!(cause = ?err, "connection error");
				}
			});
		}
	}

	pub async fn broadcast(&self, msg: Message) {

	}

}
