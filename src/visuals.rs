use crate::tetris::*;
use bevy::prelude::*;
use core::ops::Deref;

#[derive(Component, Clone)]
pub struct OwnTile;

#[derive(Component, Clone)]
pub struct OtherTile;

pub fn draw_falling(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    own_board: Res<OwnTetrisBoard>,
    own_query: Query<Entity, With<FallingTile>>,
    own_tiles: Res<FallingTiles>,
) {
    let mut draw_tiles = |tiles: &FallingTiles, board: &TetrisBoard, despawn: Vec<Entity>| {
        for e in despawn {
            commands.entity(e).despawn();
        }
        for (pos, tile) in tiles.iter() {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("tetris_tile.png"),
                    transform: Transform::from_translation(board.get_position(*pos)),
                    sprite: Sprite {
                        color: tile.color,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                FallingTile,
            ));
        }
    };

    if own_tiles.is_changed() {
        draw_tiles(
            own_tiles.deref(),
            own_board.deref(),
            own_query.iter().collect(),
        );
    }
}

pub fn draw_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    own_query: Query<Entity, With<OwnTile>>,
    own_board: Res<OwnTetrisBoard>,
    other_query: Query<Entity, With<OtherTile>>,
    other_board: Res<OtherTetrisBoard>,
) {
    if own_board.is_changed() {
        own_query.iter().for_each(|e| commands.entity(e).despawn());
        spawn_tiles(
            own_board.deref(),
            &mut commands,
            asset_server.load("tetris_tile.png"),
            OwnTile,
        );
    }

    if other_board.is_changed() {
        other_query
            .iter()
            .for_each(|e| commands.entity(e).despawn());
        spawn_tiles(
            other_board.deref(),
            &mut commands,
            asset_server.load("tetris_tile.png"),
            OtherTile,
        );
    }
}

fn spawn_tiles<T: Component + Clone>(
    board: &TetrisBoard,
    commands: &mut Commands,
    texture: Handle<Image>,
    comp: T,
) {
    for (col, l) in board.tiles.iter().enumerate() {
        for (row, t) in l.iter().enumerate() {
            if let Some(tile) = t {
                let board_position = [col as i32, row as i32].into();
                commands.spawn((
                    SpriteBundle {
                        texture: texture.clone(),
                        transform: Transform::from_translation(board.get_position(board_position)),
                        sprite: Sprite {
                            color: tile.color,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    comp.clone(),
                ));
            }
        }
    }
}
