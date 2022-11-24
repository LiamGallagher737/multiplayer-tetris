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
    stream.set_nonblocking(true).expect("Failed to enable non-blocking mode");
    commands.insert_resource(NetworkResource {
        stream,
    });
}

fn read_messages(mut network: ResMut<NetworkResource>) {
    loop {
        let mut len = [0; 1];
        match network.stream.read_exact(&mut len) {
            Err(_) => return,
            _ => {},
        };
    
        let mut buf = vec![0; len[0] as usize];
        network.stream.read_exact(&mut buf).expect("Failed reading body");
    
        let message = bincode::deserialize::<ClientMessage>(&buf[..]).expect("Failed deserializing message");
    
        println!("{:#?}", message);
    }
}
