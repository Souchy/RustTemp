
use std::{error::Error, sync::Arc};

use tokio::{net::TcpListener, sync::Mutex};
use tracing::{debug, error, info, instrument};
use crate::net::client::Client;

use super::handler::Pipeline;


struct Server {
	pipeline: Pipeline
}

impl Server {
	fn new(pipeline: Pipeline) -> Self {
		Self {
			pipeline
		}
	}
	async fn run(&self, addr: String) -> Result<(), Box<dyn Error>> {
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
	
			let (r, w) = socket.into_split(); 
			let reader = Arc::new(Mutex::new(r)); 
			let writer = Arc::new(Mutex::new(w)); 

			let client: Client = Client::new(reader, writer, self.pipeline.clone());
	
			tokio::spawn(async move {
				if let Err(err) = client.run().await {
					error!(cause = ?err, "connection error");
				}
			});
		}
	}
}
