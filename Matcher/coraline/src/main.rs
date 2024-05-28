
use teal::net::client::Client;
use teal::net::handler::MessageHandlers;
use teal::net::message::MessageScript;
use teal::net::messages::chat::ChatMsg;
use teal::net::messages::ping::PingMsg;
use teal::net::messages::pong::PongMsg;

use std::error::Error;
use std::sync::Arc;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut reg = MessageHandlers::new();
    reg.register(ChatMsg::uid(), ChatMsg::deserialize);
    reg.register(PingMsg::uid(), PingMsg::deserialize);
    reg.register(PongMsg::uid(), PongMsg::deserialize);

    let client: Client = Client::new_connection("127.0.0.1:8080", Arc::new(reg)).await?;

    let client_ref = Arc::new(client);
    let client_ref2 = client_ref.clone();

    let t1 = tokio::spawn(async move {
        println!("t1 start");
        client_ref.run().await.unwrap();
    });
    let t2 = tokio::spawn(async move {
        println!("t2 start");
        let chat = ChatMsg {
            channel: String::from("general"),
            text: String::from("hello")
        };
        chat.send(&client_ref2).await.ok();
        
        client_ref2.send(PingMsg::new()).await.ok();
    });

    t2.await?;
    let out = t1.await.unwrap();
    // tokio::join!(t1);

    Ok(())
}

