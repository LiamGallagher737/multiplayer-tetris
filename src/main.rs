use bevy::prelude::*;
use bevy::{core_pipeline::bloom::BloomSettings, time::FixedTimestep};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use tetris::*;

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

        .insert_resource(TetrisPieceBuffer::new())
        .insert_resource(OwnTetrisBoard(TetrisBoard::new([-60.0, 0.0].into())))
        .insert_resource(OtherTetrisBoard(TetrisBoard::new([60.0, 0.0].into())))

        .add_startup_system(setup_scene)
        .add_startup_system(spawn_piece)

        .add_system(player_input)
        .add_system(move_piece)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(tetris_gravity)
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
    Fall,     // Normal Falling
    SoftDrop, // Player Fall
    HardDrop, // Tp to Bottom
    RotateLeft,
    RotateRight,
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    own_tetris_board: Res<OwnTetrisBoard>,
    other_tetris_board: Res<OtherTetrisBoard>,
) {
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

    let mut spawn_board = |board: &TetrisBoard| {
        for x in 0..board.tiles.len() {
            for y in 0..board.tiles[0].len() {
                let position = board.get_position(x as u8, y as u8);
                commands.spawn(SpriteBundle {
                    texture: asset_server.load("tetris_tile.png"),
                    transform: Transform::from_translation(position),
                    sprite: Sprite {
                        color: Color::hsla(100.0, 0.0, 0.2, 0.4),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }
    };

    spawn_board(own_tetris_board.as_ref());
    spawn_board(other_tetris_board.as_ref());
}

fn spawn_piece(
    mut commands: Commands,
    mut buf: ResMut<TetrisPieceBuffer>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = thread_rng();
    let color = *TETRIS_COLORS.choose(&mut rng).unwrap();
    let piece = buf.pop();

    for (y, v) in piece.tiles.iter().enumerate() {
        for x in v
            .iter()
            .enumerate()
            .filter_map(|(i, t)| if *t { Some(i) } else { None })
        {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("tetris_tile.png"),
                    sprite: Sprite {
                        color,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                FallingPiece {
                    origin: piece.orgin,
                    board_positon: [x as i32 + 3, -(y as i32)].into(),
                },
            ));
        }
    }
}

#[derive(Component)]
struct FallingPiece {
    origin: Vec2,
    board_positon: IVec2,
}

fn move_piece(
    mut query: Query<(&mut Transform, &mut FallingPiece), With<FallingPiece>>,
    mut move_events: EventReader<TetrisMoveEvent>,
    board: Res<OwnTetrisBoard>,
) {
    for m in move_events.iter() {
        for (_, mut p) in query.iter_mut() {
            match m {
                TetrisMove::Left => p.board_positon.x -= 1,
                TetrisMove::Right => p.board_positon.x += 1,
                TetrisMove::Fall => p.board_positon.y -= 1,
                TetrisMove::SoftDrop => p.board_positon.y -= 1,
                TetrisMove::RotateLeft => {},
                TetrisMove::RotateRight => {},
                TetrisMove::HardDrop => {},
            };
        }
    }
    for (mut t, p) in query.iter_mut() {
        t.translation = board.get_position(p.board_positon.x as u8, -p.board_positon.y as u8)
    }
}

fn tetris_gravity(mut move_events: EventWriter<TetrisMoveEvent>) {
    move_events.send(TetrisMove::Fall);
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
