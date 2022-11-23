use std::error::Error;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use shared::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".into());

    let listener = TcpListener::bind(&address).await?;
    println!("Listening on: {}", address);

    loop {
        let (mut socket, client) = listener.accept().await?;

        tokio::spawn(async move {
            println!("Connection established with {}", client);
            let buf = bincode::serialize(&vec![ClientMessage::StartGame]).expect("Failed serializing messages");
            socket.write_all(&buf[..]).await.expect("Failed to write data to socket");
        });
    }
}