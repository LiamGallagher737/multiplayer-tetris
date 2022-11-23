use bevy::prelude::*;
use std::{io::prelude::*, time::Duration};

fn main() {

    let mut stream = std::net::TcpStream::connect("127.0.0.1:8080").unwrap();

    let mut buf = vec![];
    stream.read_to_end(&mut buf).unwrap();
    dbg!(buf);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Tetris Pong".into(),
                ..default()
            },
            ..Default::default()
        }))
        .run();
}


