use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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
        .add_system(player_input)
        .add_event::<TetrisMoveEvent>()
        .run();
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum GameMode {
    #[default]
    Normal,
    Hyper,
    Swap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TetrisMove {
    Left,
    Right,
    Drop,     // Normal Falling
    SoftDrop, // Player Fall
    HardDrop, // Tp to Bottom
    RotateLeft,
    RotateRight,
}

pub type TetrisMoveEvent = TetrisMove;

fn player_input(keys: Res<Input<KeyCode>>, mut move_events: EventWriter<TetrisMoveEvent>) {
    // Based on this post https://www.reddit.com/r/Tetris/comments/8viwld/comment/e5kcgr7/?utm_source=share&utm_medium=web2x&context=3
    if keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up) {
        move_events.send(TetrisMove::HardDrop);
    }
    if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down) {
        move_events.send(TetrisMove::SoftDrop);
    }
    if keys.just_pressed(KeyCode::A) || keys.just_pressed(KeyCode::Left) {
        move_events.send(TetrisMove::Left);
    }
    if keys.just_pressed(KeyCode::D) || keys.just_pressed(KeyCode::Right) {
        move_events.send(TetrisMove::Right);
    }
    if keys.just_pressed(KeyCode::Q) || keys.just_pressed(KeyCode::Z) {
        move_events.send(TetrisMove::RotateLeft);
    }
    if keys.just_pressed(KeyCode::E) || keys.just_pressed(KeyCode::X) {
        move_events.send(TetrisMove::RotateRight);
    }
}
