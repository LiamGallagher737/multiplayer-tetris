use bevy::prelude::*;
use lazy_static::{__Deref, lazy_static};
use rand::{seq::SliceRandom, thread_rng};

// pub type OwnTetrisBoard = TetrisBoard;
// pub type OtherTetrisBoard = TetrisBoard;

#[derive(Resource, Deref)]
pub struct OwnTetrisBoard(pub TetrisBoard);

#[derive(Resource, Deref)]
pub struct OtherTetrisBoard(pub TetrisBoard);

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
    pub fn get_position(&self, x: u8, y: u8) -> Vec3 {
        [
            (x as f32 * 8.0) - (5.0 * 8.0) + 4.0 + self.offset.x,
            -(y as f32 * 8.0) + (10.0 * 8.0) - 4.0 + self.offset.y,
            0.0,
        ]
        .into()
    }
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

#[derive(Clone, Copy)]
pub struct TetrisTile {
    color: Color,
}

#[derive(Clone)]
pub struct TetrisPiece {
    pub orgin: Vec2, // Starting froom top left of tiles
    pub rotate: bool,
    pub tiles: Vec<Vec<bool>>,
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
            orgin: [1.0, 0.0].into(),
            rotate: true,
            tiles: [
                [true, true, true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1.0, -1.0].into(),
            rotate: true,
            tiles: [
                [true, false, false].into(),
                [true, true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1.0, -1.0].into(),
            rotate: true,
            tiles: [
                [false, false, true].into(),
                [true, true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [0.5, 0.5].into(),
            rotate: false,
            tiles: [
                [true, true].into(),
                [true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1.0, -1.0].into(),
            rotate: true,
            tiles: [
                [false, true, true].into(),
                [true, true, false].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1.0, -1.0].into(),
            rotate: true,
            tiles: [
                [true, true, false].into(),
                [false, true, true].into(),
            ].into(),
        },
        TetrisPiece {
            orgin: [1.0, -1.0].into(),
            rotate: true,
            tiles: [
                [true, true, true].into(),
                [false, true, false].into(),
            ].into(),
        },
    ];
}
