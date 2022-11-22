use std::error::Error;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
            println!("Received request from {}", client);
            socket.write_all(&[12, 82, 4]).await.expect("Failed to write data to socket");
        });
    }
}