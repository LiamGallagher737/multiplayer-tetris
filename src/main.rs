use bevy::prelude::*;
use bevy::{core_pipeline::bloom::BloomSettings, time::FixedTimestep};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use tetris::{tetris_peices, OwnTetrisBoard, TetrisPieceBuffer};

mod network;
mod tetris;

#[rustfmt::skip]
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("010d13").unwrap()))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Tetris Pong".into(),
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    },
                    ..Default::default()
                })
                .set(ImagePlugin::default_linear()),
        )

        .add_plugin(network::NetworkPlugin)
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())

        .add_startup_system(setup_scene)
        .add_startup_system(setup_game)

        .add_system(player_input)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(2.0))
                .with_system(spawn_piece)
        )

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

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.25,
                ..Default::default()
            },
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            ..Default::default()
        },
        BloomSettings {
            threshold: 0.3,
            intensity: 1.5,
            ..Default::default()
        },
    ));

    commands.insert_resource(OwnTetrisBoard::empty());

    for x in 0..10 {
        for y in 0..20 {
            let position = [
                (x as f32 * 8.0) - (5.0 * 8.0),
                -(y as f32 * 8.0) + (10.0 * 8.0),
                0.0,
            ]
            .into();
            commands.spawn(SpriteBundle {
                texture: asset_server.load("tetris_tile.png"),
                transform: Transform::from_translation(position),
                sprite: Sprite {
                    color: Color::hsla(100.0, 0.0, 0.2, 0.2),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

fn setup_game(mut commands: Commands) {
    commands.insert_resource(TetrisPieceBuffer::new());
}

fn spawn_piece(
    mut commands: Commands,
    mut buf: ResMut<TetrisPieceBuffer>,
    query: Query<Entity, With<Sprite>>,
    asset_server: Res<AssetServer>,
) {
    for e in query.iter() {
        commands.entity(e).despawn();
    }
    for (y, v) in buf.pop().tiles.iter().enumerate() {
        for x in v
            .iter()
            .enumerate()
            .filter_map(|(i, t)| if *t { Some(i) } else { None })
        {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("tetris_tile.png"),
                transform: Transform::from_xyz(x as f32 * 8.0, y as f32 * 8.0, 0.0),
                sprite: Sprite {
                    color: Color::hsl(100.0, 0.7, 0.8),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
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
