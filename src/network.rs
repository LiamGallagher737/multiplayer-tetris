use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, IntoConditionalSystem};
use local_ip_address::local_ip;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    tetris::{OtherTetrisBoard, OwnTetrisBoard, TetrisTile},
    GameMode,
};

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(HostAddress::default());

        app.add_enter_system(NetworkState::Host, setup_host);
        app.add_enter_system(NetworkState::Client, setup_client);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(NetworkState::Host)
                .with_system(check_for_connections.run_unless_resource_exists::<ClientResource>())
                .into(),
        );

        app.add_system(receive_messages.run_if_resource_exists::<ClientResource>());
        app.add_system(send_board_updates.run_if_resource_exists::<ClientResource>());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum NetworkState {
    #[default]
    None,
    Host,
    Client,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct HostAddress(pub String);

#[derive(Resource)]
struct HostResource {
    listener: TcpListener,
}

#[derive(Resource)]
pub struct ClientResource {
    stream: TcpStream,
}

#[derive(Serialize, Deserialize, Debug)]
enum HostMessage {
    Mode(GameMode),
}

#[derive(Serialize, Deserialize, Debug)]
enum ClientMessage {
    BoardUpdate([[Option<TetrisTile>; 20]; 10]),
}

fn setup_host(mut commands: Commands) {
    let ip = local_ip()
        .expect("Failed to get computers local Ip address")
        .to_string();
    let addr = format!("{ip}:8080");
    let listener = TcpListener::bind(addr.clone()).expect("Failed creating TCP listener");
    listener
        .set_nonblocking(true)
        .expect("Failed to enable non-blocking mode");
    println!("Hosting TCP server at {addr}");
    commands.insert_resource(HostResource { listener });
}

fn setup_client(mut commands: Commands, ip: Res<HostAddress>) {
    // let ip = std::env::args()
    //     .nth(2)
    //     .expect("Please provide a IP to connect to");
    let addr = format!("{}:8080", ip.0);
    let stream = TcpStream::connect(addr.clone()).expect("Failed to connect to TCP server");
    stream
        .set_nonblocking(true)
        .expect("Failed to enable non-blocking mode");
    println!("Connected to TCP server at {addr}");
    commands.insert_resource(ClientResource { stream });
}

fn check_for_connections(mut commands: Commands, host: Res<HostResource>) {
    if let Some(Ok(stream)) = host.listener.incoming().next() {
        // used to be .nth(0)
        println!("Client connected from {}", stream.peer_addr().unwrap());
        commands.insert_resource(ClientResource { stream });
    }
}

fn receive_messages(mut client: ResMut<ClientResource>, mut other_board: ResMut<OtherTetrisBoard>) {
    for message in deserialize_messages::<ClientMessage>(&mut client.stream) {
        match message {
            ClientMessage::BoardUpdate(e) => {
                other_board.tiles = e;
            }
        }
    }
}

fn send_board_updates(board: Res<OwnTetrisBoard>, mut client: ResMut<ClientResource>) {
    if !board.is_changed() {
        return;
    }
    let buf = serialize_message(ClientMessage::BoardUpdate(board.0.tiles.to_owned()));
    client
        .stream
        .write_all(&buf)
        .expect("Failed to send board update");
}

fn serialize_message<T: Serialize>(msg: T) -> Vec<u8> {
    let mut buf = bincode::serialize(&msg).expect("Failed serializing message");
    let len = (buf.len() as u16).to_be_bytes();
    buf.insert(0, len[0]);
    buf.insert(1, len[1]);
    buf
}

fn deserialize_messages<T: DeserializeOwned>(stream: &mut TcpStream) -> Vec<T> {
    let mut messages = vec![];
    loop {
        let mut len_bytes = [0; 2];
        if let Err(_) = stream.read_exact(&mut len_bytes) {
            break;
        }
        let len = u16::from_be_bytes(len_bytes);
        let mut buf = vec![0; len as usize];
        stream.read_exact(&mut buf).expect("Failed reading body");

        let message = bincode::deserialize::<T>(&buf[..]).expect("Failed deserializing message");
        messages.push(message);
    }
    messages
}
