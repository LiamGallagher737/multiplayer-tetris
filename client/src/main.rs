use bevy::prelude::*;

mod network;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Tetris Pong".into(),
                ..default()
            },
            ..Default::default()
        }))
        .add_plugin(network::NetworkPlugin)
        .run();
}
