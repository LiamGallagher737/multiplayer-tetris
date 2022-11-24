use std::error::Error;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use shared::*;
use tokio::time::sleep;

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
            loop {

                let mut buf = bincode::serialize(&ClientMessage::GameStart).expect("Failed serializing messages");
                buf.insert(0, buf.len() as u8);
                socket.write(&buf[..]).await.expect("Failed to write data to socket");

                let mut buf = bincode::serialize(&ClientMessage::GameEnd(7)).expect("Failed serializing messages");
                buf.insert(0, buf.len() as u8);
                socket.write(&buf[..]).await.expect("Failed to write data to socket");

                println!("Sent Data");
                sleep(Duration::from_secs(2)).await;
            }
        });
    }
}