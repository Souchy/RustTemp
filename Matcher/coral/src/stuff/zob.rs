use tracing::{debug, error, info, instrument};

pub struct Zob {

}

impl Zob {
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            // Asynchronously wait for an inbound socket.
            // let (mut socket, _) = listener.accept().await?;
            let socket = self.accept().await?;
    
            // And this is where much of the magic of this server happens. We
            // crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the
            // `tokio::spawn` function to execute the work in the background.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to be processed concurrently.
    
            tokio::spawn(async move {
                let mut buf = vec![0; 1024];
    
                // In a loop, read data from the socket and write the data back.
                loop {
                    let n = socket
                        .read(&mut buf)
                        .await
                        .expect("failed to read data from socket");
    
                    if n == 0 {
                        return;
                    }
    
                    let st = std::str::from_utf8(&buf).unwrap();
                    println!("received: {}", st);
    
                    socket
                        .write_all(&buf[0..n])
                        .await
                        .expect("failed to write data to socket");
                }
            });
        }
    }
}

