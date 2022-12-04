use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

// This post wwas a big help
// https://stackoverflow.com/a/38596291

pub const SHAPES: [TetrisPiece; 7] = [
    // degrees          0       90      180     270
    TetrisPiece::new([0x4640, 0x0E40, 0x4C40, 0x4E00]), // 'T'
    TetrisPiece::new([0x8C40, 0x6C00, 0x8C40, 0x6C00]), // 'S'
    TetrisPiece::new([0x4C80, 0xC600, 0x4C80, 0xC600]), // 'Z'
    TetrisPiece::new([0x4444, 0x0F00, 0x4444, 0x0F00]), // 'I'
    TetrisPiece::new([0x44C0, 0x8E00, 0xC880, 0xE200]), // 'J'
    TetrisPiece::new([0x88C0, 0xE800, 0xC440, 0x2E00]), // 'L'
    TetrisPiece::new([0xCC00, 0xCC00, 0xCC00, 0xCC00]), // 'O'
];

pub const COLORS: [Color; 6] = [
    Color::hsl(0.0, 0.7, 0.8),
    Color::hsl(50.0, 0.7, 0.8),
    Color::hsl(100.0, 0.7, 0.8),
    Color::hsl(175.0, 0.7, 0.8),
    Color::hsl(240.0, 0.7, 0.8),
    Color::hsl(300.0, 0.7, 0.8),
];

#[derive(Component)]
pub struct FallingTile;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TetrisTile {
    pub color: Color,
}

#[derive(Resource)]
pub struct CurrentPiece {
    pub piece: TetrisPiece,
    pub position: IVec2,
    pub rotation: usize,
    pub tiles: Vec<(IVec2, TetrisTile)>,
}

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
    pub const fn new(offset: Vec2) -> Self {
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

#[derive(Clone, Debug)]
pub struct TetrisPiece {
    pub data: [u16; 4],
}

impl TetrisPiece {
    pub const fn new(data: [u16; 4]) -> Self {
        Self { data }
    }
    pub const fn value(&self, rotation: usize, x: u8, y: u8) -> bool {
        self.data[rotation % 4] & (0x8000 >> (y * 4 + x)) != 0
    }
}

impl std::fmt::Display for TetrisPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..4 {
            for y in 0..4 {
                for x in 0..4 {
                    write!(f, "{}, ", self.value(r, x, y) as u8)?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Resource)]
pub struct TetrisPieceBuffer {
    pieces: Vec<TetrisPiece>,
}

impl TetrisPieceBuffer {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut pieces = SHAPES.to_vec();
        pieces.shuffle(&mut rng);
        Self { pieces }
    }
    pub fn pop(&mut self) -> TetrisPiece {
        if self.pieces.is_empty() {
            let mut rng = thread_rng();
            let mut pieces = SHAPES.to_vec();
            pieces.shuffle(&mut rng);
            self.pieces = pieces;
        }
        self.pieces.pop().unwrap()
    }
}

pub fn spawn_piece(mut commands: Commands, mut buf: ResMut<TetrisPieceBuffer>) {
    let mut rng = thread_rng();
    let color = COLORS.choose(&mut rng).unwrap();
    let piece = buf.pop();

    let mut current_piece = CurrentPiece {
        piece: piece.clone(),
        position: [3, 0].into(),
        rotation: 0,
        tiles: vec![],
    };

    for x in 0..4 {
        for y in 0..4 {
            if piece.value(0, x, y) {
                let board_position = [x as i32 + 3, y as i32].into();
                current_piece.tiles.push((
                    board_position,
                    TetrisTile {
                        color: color.to_owned(),
                    },
                ));
            }
        }
    }

    commands.insert_resource(current_piece);
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

    let _points = match is_line.len() {
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
