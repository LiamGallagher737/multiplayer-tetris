use crate::tetris::*;
use bevy::prelude::*;
use core::ops::Deref;

pub(crate) fn draw_falling(
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

pub(crate) fn draw_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    own_query: Query<Entity, With<Tile>>,
    own_board: Res<OwnTetrisBoard>,
    // other_board: Res<OtherTetrisBoard>,
) {
    let mut draw_board = |board: &TetrisBoard, despawn: Vec<Entity>| {
        for e in despawn {
            commands.entity(e).despawn();
        }
        for (col, l) in board.tiles.iter().enumerate() {
            for (row, t) in l.iter().enumerate() {
                if let Some(tile) = t {
                    let board_position = [col as i32, row as i32].into();
                    commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load("tetris_tile.png"),
                            transform: Transform::from_translation(
                                board.get_position(board_position),
                            ),
                            sprite: Sprite {
                                color: tile.color,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        Tile,
                    ));
                }
            }
        }
    };

    if own_board.is_changed() {
        draw_board(own_board.deref(), own_query.iter().collect());
    }
}
