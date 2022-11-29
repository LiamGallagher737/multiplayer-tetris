use bevy::prelude::*;
use bevy::{core_pipeline::bloom::BloomSettings, time::FixedTimestep};
use iyes_loopless::prelude::IntoConditionalSystem;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use tetris::*;

mod movement;
mod network;
mod tetris;
mod visuals;

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
                .set(ImagePlugin::default_nearest()),
        )

        .add_plugin(bevy_editor_pls::EditorPlugin)
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())

        .add_plugin(network::NetworkPlugin)

        .insert_resource(TetrisPieceBuffer::new())
        .insert_resource(OwnTetrisBoard(TetrisBoard::new([-60.0, 0.0].into())))
        .insert_resource(OtherTetrisBoard(TetrisBoard::new([60.0, 0.0].into())))

        .add_startup_system(setup_scene)
        
        .add_system(movement::player_input)
        .add_system(movement::move_piece.run_if_resource_exists::<FallingTiles>())
        .add_system(spawn_piece.run_unless_resource_exists::<FallingTiles>())
        .add_system(visuals::draw_falling.run_if_resource_exists::<FallingTiles>())
        .add_system(visuals::draw_tiles)
        .add_system(tetris::clear_lines.run_if_resource_removed::<FallingTiles>()) // .run_if_resource_removed::<FallingTiles>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0))
                .with_system(movement::tetris_gravity)
        )

        .add_event::<movement::TetrisMoveEvent>()

        .run();
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum GameMode {
    #[default]
    Normal,
    Hyper,
    Swap,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TetrisMove {
    Left,
    Right,
    Fall,
    RotateLeft,
    RotateRight,
}

fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    own_board: Res<OwnTetrisBoard>,
    other_board: Res<OtherTetrisBoard>,
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
            intensity: 2.0,
            ..Default::default()
        },
    ));

    let mut spawn_board = |board: &TetrisBoard| {
        commands.spawn(SpatialBundle::default()).with_children(|p| {
            for x in 0..board.tiles.len() {
                for y in 0..board.tiles[0].len() {
                    let position = board.get_position([x as i32, y as i32].into());
                    p.spawn(SpriteBundle {
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
        });
    };

    spawn_board(own_board.as_ref());
    spawn_board(other_board.as_ref());
}

fn spawn_piece(mut commands: Commands, mut buf: ResMut<TetrisPieceBuffer>) {
    let mut rng = thread_rng();
    let color = *TETRIS_COLORS.choose(&mut rng).unwrap();
    let piece = buf.pop();

    let mut falling_tiles = vec![];
    for (y, v) in piece.tiles.iter().enumerate() {
        for x in v
            .iter()
            .enumerate()
            .filter_map(|(i, t)| if *t { Some(i) } else { None })
        {
            let board_position = [x as i32 + 3, y as i32].into();
            falling_tiles.push((board_position, TetrisTile { color }));
        }
    }
    commands.insert_resource(FallingTiles(falling_tiles));
}
