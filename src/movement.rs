use crate::{tetris::*, TetrisMove};
use bevy::prelude::*;

pub fn move_piece(
    mut commands: Commands,
    mut move_events: EventReader<TetrisMoveEvent>,
    mut falling_tiles: ResMut<FallingTiles>,
    mut board: ResMut<OwnTetrisBoard>,
) {
    let mut stop_falling = false;
    'move_loop: for m in move_events.iter() {
        // Check if move allowed
        for (p, _) in falling_tiles.iter() {
            match m {
                TetrisMove::Left => {
                    if !board.tile_empty(*p - IVec2::X) {
                        continue 'move_loop;
                    }
                }
                TetrisMove::Right => {
                    if !board.tile_empty(*p + IVec2::X) {
                        continue 'move_loop;
                    }
                }
                TetrisMove::Fall => {
                    if !board.tile_empty(*p + IVec2::Y) {
                        stop_falling = true;
                        break 'move_loop;
                    }
                }
                _ => {}
            }
        }

        // If check passes, move the tiles
        for tile in falling_tiles.0.iter_mut() {
            match m {
                TetrisMove::Left => tile.0 -= IVec2::X,
                TetrisMove::Right => tile.0 += IVec2::X,
                TetrisMove::Fall => tile.0 += IVec2::Y,
                _ => {}
            }
        }
    }

    move_events.clear();

    if stop_falling {
        for (pos, tile) in falling_tiles.iter() {
            board.set(*pos, Some(*tile));
        }
        commands.remove_resource::<FallingTiles>();
    }
}

pub fn tetris_gravity(mut move_events: EventWriter<TetrisMoveEvent>) {
    move_events.send(TetrisMove::Fall);
}

pub type TetrisMoveEvent = TetrisMove;

pub fn player_input(keys: Res<Input<KeyCode>>, mut move_events: EventWriter<TetrisMoveEvent>) {
    // Based on this post https://www.reddit.com/r/Tetris/comments/8viwld/comment/e5kcgr7/?utm_source=share&utm_medium=web2x&context=3
    if keys.just_pressed(KeyCode::W) || keys.just_pressed(KeyCode::Up) {
        move_events.send_batch([TetrisMove::Fall; 20])
    }
    if keys.just_pressed(KeyCode::S) || keys.just_pressed(KeyCode::Down) {
        move_events.send(TetrisMove::Fall);
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
