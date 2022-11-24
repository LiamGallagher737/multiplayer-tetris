#![allow(unused_imports)] // temporary

use std::error::Error;
use std::task::Context;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use shared::*;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".into());

    let listener = TcpListener::bind(&address).await?;
    println!("Listening on: {}", address);

    // let mut sockets = vec![];

    loop {
        let (mut socket, client) = listener.accept().await?;

        // listener.

        tokio::spawn(async move {
            println!("Connection established with {}", client);
            loop {

                // let mut buf = bincode::serialize(&ClientMessage::GameStart).expect("Failed serializing messages");
                // buf.insert(0, buf.len() as u8);
                // socket.write(&buf[..]).await.expect("Failed to write data to socket");

                // let mut buf = bincode::serialize(&ClientMessage::GameEnd(7)).expect("Failed serializing messages");
                // buf.insert(0, buf.len() as u8);
                // socket.write(&buf[..]).await.expect("Failed to write data to socket");

                // println!("Sent Data");
                // sleep(Duration::from_secs(2)).await;

                let mut len = [0; 1];
                match socket.read_exact(&mut len).await {
                    Err(_) => continue,
                    _ => {},
                };
            
                let mut buf = vec![0; len[0] as usize];
                socket.read_exact(&mut buf).await.expect("Failed reading body");
            
                let message = bincode::deserialize::<ServerMessage>(&buf[..]).expect("Failed deserializing message");

                println!("Message from {client}: {:#?}", message);

                match message {
                    ServerMessage::TetrisMove(e) => {
                        let bin = serialize_message(ClientMessage::OtherTetrisMove(e));
                    },
                }

            }
        });
    }
}
