use bevy::prelude::*;
use iyes_loopless::prelude::IntoConditionalSystem;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{GameMode, TetrisMove, TetrisMoveEvent};

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let t = std::env::args()
            .nth(1)
            .expect("Please choose HOST or CLIENT")
            .to_lowercase();
        match t.as_str() {
            "host" => {
                app.add_startup_system(setup_host);
                app.add_system(
                    check_for_connections.run_unless_resource_exists::<ClientResource>(),
                );
            }
            "client" => {
                app.add_startup_system(setup_client);
            }
            _ => panic!("Please choose HOST or CLIENT"),
        };
        app.add_system(receive_messages.run_if_resource_exists::<ClientResource>());
        app.add_system(send_move_events.run_if_resource_exists::<ClientResource>());
    }
}

#[derive(Resource)]
struct HostResource {
    listener: TcpListener,
}

#[derive(Resource)]
struct ClientResource {
    stream: TcpStream,
}

#[derive(Serialize, Deserialize, Debug)]
enum HostMessage {
    Mode(GameMode),
}

#[derive(Serialize, Deserialize, Debug)]
enum ClientMessage {
    Move(TetrisMove),
}

fn setup_host(mut commands: Commands) {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed creating TCP listener");
    listener
        .set_nonblocking(true)
        .expect("Failed to enable non-blocking mode");
    println!("Hosting TCP server at 127.0.0.1:8080");
    commands.insert_resource(HostResource { listener });
}

fn setup_client(mut commands: Commands) {
    let stream = TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to TCP server");
    stream
        .set_nonblocking(true)
        .expect("Failed to enable non-blocking mode");
    println!("Connected to TCP server at 127.0.0.1:8080");
    commands.insert_resource(ClientResource { stream });
}

fn check_for_connections(mut commands: Commands, host: Res<HostResource>) {
    if let Some(Ok(stream)) = host.listener.incoming().nth(0) {
        println!("Client connected from {}", stream.peer_addr().unwrap());
        commands.insert_resource(ClientResource { stream });
    }
}

fn receive_messages(mut client: ResMut<ClientResource>) {
    for message in deserialize_messages::<ClientMessage>(&mut client.stream) {
        match message {
            ClientMessage::Move(m) => {
                dbg!(m);
            }
        }
    }
}

fn send_move_events(
    mut move_events: EventReader<TetrisMoveEvent>,
    mut client: ResMut<ClientResource>,
) {
    for m in move_events.iter() {
        let buf = serialize_message(ClientMessage::Move(m.to_owned()));
        client
            .stream
            .write(&buf)
            .expect("Failed to send movement to server");
    }
}

fn serialize_message<T: Serialize>(msg: T) -> Vec<u8> {
    let mut buf = bincode::serialize(&msg).expect("Failed serializing message");
    buf.insert(0, buf.len() as u8);
    buf
}

fn deserialize_messages<T: DeserializeOwned>(stream: &mut TcpStream) -> Vec<T> {
    let mut messages = vec![];
    loop {
        let mut len = [0; 1];
        match stream.read_exact(&mut len) {
            Err(_) => break,
            _ => {}
        };

        let mut buf = vec![0; len[0] as usize];
        stream.read_exact(&mut buf).expect("Failed reading body");

        let message = bincode::deserialize::<T>(&buf[..]).expect("Failed deserializing message");
        messages.push(message);
    }
    messages
}