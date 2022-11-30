use crate::{tetris::*, TetrisMove};
use bevy::prelude::*;

pub fn move_piece(
    mut commands: Commands,
    mut move_events: EventReader<TetrisMoveEvent>,
    mut current_piece: ResMut<CurrentPiece>,
    mut board: ResMut<OwnTetrisBoard>,
) {
    let mut stop_falling = false;
    'move_loop: for m in move_events.iter() {
        // Check if move allowed
        for (p, _) in current_piece.tiles.iter() {
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

        if let TetrisMove::RotateLeft | TetrisMove::RotateRight = m {
            // Check if rotaton allowed
            let rotation = current_piece.rotation
                + match m {
                    TetrisMove::RotateLeft => 3,
                    TetrisMove::RotateRight => 1,
                    _ => 0,
                };

            for x in 0..4 {
                for y in 0..4 {
                    if current_piece.piece.value(rotation, x, y) {
                        if !board
                            .tile_empty(IVec2::new(x as i32, y as i32) + current_piece.position)
                        {
                            continue 'move_loop;
                        }
                    }
                }
            }

            // If check passes, move the tiles
            let color = current_piece.tiles[0].1.color;
            current_piece.tiles.clear();

            for x in 0..4 {
                for y in 0..4 {
                    if current_piece.piece.value(rotation, x, y) {
                        let board_position =
                            IVec2::new(x as i32, y as i32) + current_piece.position;
                        current_piece
                            .tiles
                            .push((board_position, TetrisTile { color }));
                    }
                }
            }

            current_piece.rotation = rotation;

            continue 'move_loop;
        }

        // If check passes, move the tiles
        for tile in current_piece.tiles.iter_mut() {
            match m {
                TetrisMove::Left => tile.0 -= IVec2::X,
                TetrisMove::Right => tile.0 += IVec2::X,
                TetrisMove::Fall => tile.0 += IVec2::Y,
                _ => {}
            }
        }
        match m {
            TetrisMove::Left => current_piece.position -= IVec2::X,
            TetrisMove::Right => current_piece.position += IVec2::X,
            TetrisMove::Fall => current_piece.position += IVec2::Y,
            _ => {}
        }
    }

    move_events.clear();

    if stop_falling {
        for (pos, tile) in current_piece.tiles.iter() {
            board.set(*pos, Some(*tile));
        }
        commands.remove_resource::<CurrentPiece>();
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
