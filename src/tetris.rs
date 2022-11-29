use bevy::prelude::*;
use lazy_static::{__Deref, lazy_static};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct FallingTile;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TetrisTile {
    pub color: Color,
}

#[derive(Resource, Deref, DerefMut)]
pub struct FallingTiles(pub Vec<(IVec2, TetrisTile)>);

#[derive(Resource, Deref, DerefMut)]
pub struct OwnTetrisBoard(pub TetrisBoard);

#[derive(Resource, Deref, DerefMut)]
pub struct OtherTetrisBoard(pub TetrisBoard);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TetrisBoard {
    pub offset: Vec2,
    pub tiles: [[Option<TetrisTile>; 20]; 10],
}

impl TetrisBoard {
    pub fn new(offset: Vec2) -> Self {
        Self {
            offset,
            tiles: [[None; 20]; 10],
        }
    }
    pub fn set(&mut self, tile: IVec2, value: Option<TetrisTile>) {
        if let Some(e) = self.tiles.get_mut(tile.x as usize) {
            if let Some(e) = e.get_mut(tile.y as usize) {
                *e = value;
            }
        }
    }
    pub fn get_position(&self, tile: IVec2) -> Vec3 {
        [
            (tile.x as f32 * 8.0) - (5.0 * 8.0) + 4.0 + self.offset.x,
            -(tile.y as f32 * 8.0) + (10.0 * 8.0) - 4.0 + self.offset.y,
            0.0,
        ]
        .into()
    }
    pub fn tile_empty(&self, tile: IVec2) -> bool {
        if let Some(e) = self.tiles.get(tile.x as usize) {
            if let Some(e) = e.get(tile.y as usize) {
                return e.is_none();
            }
        }
        false
    }
}

#[derive(Clone)]
pub struct TetrisPiece {
    pub orgin: IVec2, // Starting froom top left of tiles
    pub rotate: bool,
    pub tiles: Vec<Vec<bool>>,
}

#[derive(Resource)]
pub struct TetrisPieceBuffer {
    pieces: Vec<TetrisPiece>,
}

impl TetrisPieceBuffer {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut pieces = TETRIS_PIECES.deref().to_owned();
        pieces.shuffle(&mut rng);
        Self { pieces }
    }
    pub fn pop(&mut self) -> TetrisPiece {
        if self.pieces.is_empty() {
            let mut rng = thread_rng();
            let mut pieces = TETRIS_PIECES.deref().to_owned();
            pieces.shuffle(&mut rng);
            self.pieces = pieces;
        }
        self.pieces.pop().unwrap()
    }
}

pub fn clear_lines(mut board: ResMut<OwnTetrisBoard>) {
    let mut is_line = [true; 20];
    for col in board.tiles {
        for (j, tile) in col.iter().enumerate() {
            if tile.is_none() {
                is_line[j] = false;
            }
        }
    }

    let points = match is_line.len() {
        1 => 40,
        2 => 100,
        3 => 30,
        4 => 1200,
        _ => 0,
    };

    for i in is_line
        .iter()
        .enumerate()
        .filter_map(|(i, l)| if *l { Some(i) } else { None })
    {
        for col in 0..board.tiles.len() {
            board.tiles[col][i] = None;
            for row in (0..i).rev() {
                let t = board.tiles[col][row];
                board.tiles[col][row] = None;
                board.tiles[col][row + 1] = t;
            }
        }
    }
}

lazy_static! {

    pub static ref TETRIS_COLORS: Vec<Color> = vec![
        Color::hsl(0.0, 0.7, 0.8),
        Color::hsl(50.0, 0.7, 0.8),
        Color::hsl(100.0, 0.7, 0.8),
        Color::hsl(175.0, 0.7, 0.8),
        Color::hsl(240.0, 0.7, 0.8),
        Color::hsl(300.0, 0.7, 0.8),
    ];

    // Based on this tetris game https://tetris.com/play-tetris
    pub static ref TETRIS_PIECES: Vec<TetrisPiece> = vec![
        TetrisPiece {
            orgin: [1, 0].into(),
            rotate: true,
            tiles: [
                [true, true, true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1, -1].into(),
            rotate: true,
            tiles: [
                [true, false, false].into(),
                [true, true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1, -1].into(),
            rotate: true,
            tiles: [
                [false, false, true].into(),
                [true, true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [0, 0].into(),
            rotate: false,
            tiles: [
                [true, true].into(),
                [true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1, -1].into(),
            rotate: true,
            tiles: [
                [false, true, true].into(),
                [true, true, false].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1, -1].into(),
            rotate: true,
            tiles: [
                [true, true, false].into(),
                [false, true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1, -1].into(),
            rotate: true,
            tiles: [
                [true, true, true].into(),
                [false, true, false].into(),
            ].into(),
        },
    ];

}
