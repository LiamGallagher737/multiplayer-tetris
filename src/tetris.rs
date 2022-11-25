use bevy::prelude::{Color, Deref, Resource, Vec2};
use rand::{seq::SliceRandom, thread_rng};

pub type OwnTetrisBoard = TetrisBoard;
pub type OtherTetrisBoard = TetrisBoard;

#[derive(Deref, Resource)]
pub struct TetrisBoard {
    board: [[Option<TetrisTile>; 20]; 10],
}

impl TetrisBoard {
    pub fn empty() -> Self {
        Self {
            board: [[None; 20]; 10],
        }
    }
}

#[derive(Resource)]
pub struct TetrisPieceBuffer {
    pieces: Vec<TetrisPiece>,
}

impl TetrisPieceBuffer {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut pieces = tetris_peices();
        pieces.shuffle(&mut rng);
        Self { pieces }
    }
    pub fn pop(&mut self) -> TetrisPiece {
        if self.pieces.is_empty() {
            let mut rng = thread_rng();
            let mut pieces = tetris_peices();
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

pub struct TetrisPiece {
    pub orgin: Vec2, // Starting froom top left of tiles
    pub rotate: bool,
    pub tiles: Vec<Vec<bool>>,
}

#[rustfmt::skip]
pub fn tetris_peices() -> Vec<TetrisPiece> {
    // Based on this tetris game https://tetris.com/play-tetris
    vec![
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
                [false, true, false].into(),
                [true, true, true].into(),
            ].into(),
        },
    ]
}
