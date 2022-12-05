use std::time::Duration;

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use iyes_loopless::prelude::{
    AppLooplessFixedTimestepExt, AppLooplessStateExt, ConditionSet, IntoConditionalSystem,
};
use network::{NetworkState, ClientResource};
use serde::{Deserialize, Serialize};
use tetris::*;

mod movement;
mod network;
mod tetris;
mod ui;
mod visuals;

#[rustfmt::skip]
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("010d13").unwrap()))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Tetris Pong".into(),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                },
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
        )

        .add_plugin(bevy_editor_pls::EditorPlugin)
        // .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())

        .add_loopless_state(GameState::Menu)
        .add_loopless_state(NetworkState::default())

        .add_plugin(ui::UiPlugin)
        .add_plugin(network::NetworkPlugin)

        .add_startup_system(setup)

        // Playing
        .add_enter_system(GameState::Playing, game_setup)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(movement::player_input)
                // .with_system(movement::move_piece.run_if_resource_exists::<CurrentPiece>())
                .with_system(visuals::draw_falling.run_if_resource_exists::<CurrentPiece>())
                .with_system(visuals::draw_tiles)
                .with_system(tetris::spawn_piece.run_unless_resource_exists::<CurrentPiece>())
                .with_system(tetris::clear_lines.run_if_resource_removed::<CurrentPiece>())
                .into()
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .run_if_resource_exists::<CurrentPiece>()
                .run_if_resource_exists::<ClientResource>()
                .with_system(movement::move_piece)
                .into()
        )

        .add_fixed_timestep(Duration::from_millis(1000), "gravity")
        .add_fixed_timestep_system_set("gravity", 0,
            ConditionSet::new()
                .run_in_state(GameState::Playing)
                .with_system(movement::tetris_gravity)
                .into()
        )

        .add_event::<movement::TetrisMoveEvent>()

        .run();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Menu,
    JoinMenu,
    Playing,
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

fn setup(mut commands: Commands) {
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
}

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let own_board = TetrisBoard::new([-60.0, 0.0].into());
    let other_board = TetrisBoard::new([60.0, 0.0].into());
    commands.insert_resource(TetrisPieceBuffer::new());
    commands.insert_resource(OwnTetrisBoard(own_board.clone()));
    commands.insert_resource(OtherTetrisBoard(other_board.clone()));

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

    spawn_board(&own_board);
    spawn_board(&other_board);
}
