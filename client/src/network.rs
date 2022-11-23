use std::net::TcpStream;
use bevy::prelude::*;
use std::io::prelude::*;
use shared::*;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_client);
        app.add_system(read_messages);
    }
}

#[derive(Resource)]
struct NetworkResource {
    stream: TcpStream,
}

fn setup_client(mut commands: Commands) {
    let stream = std::net::TcpStream::connect("127.0.0.1:8080").expect("Failed to connect to server");
    commands.insert_resource(NetworkResource {
        stream,
    });
}

fn read_messages(mut network: ResMut<NetworkResource>) {
    let mut buf = vec![];
    network.stream.read_to_end(&mut buf).expect("Failed reading stream");
    let messages: Vec<ClientMessage> = bincode::deserialize(&buf[..]).expect("Unable to deserialize stream");
    if messages.len() > 0 {
        println!("Messages: {:#?}", messages);
    }
}
