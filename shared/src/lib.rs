use serde::{Serialize, Deserialize};

pub const FALL_SPEED: f32 = 1.0;
pub const SOFT_DROP_SPEED: f32 = 2.0;
pub const HYPER_MODE_MULTIPLIER: f32 = 2.5;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    GameStart, // Start the Game
    OtherTetrisMove(TetrisMove), // Other player peice moved
    GameMode(GameMode), // Set game state
    GameEnd(u8), // End the game | u8: Winning Player
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    TetrisMove(TetrisMove),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TetrisMove {
    Left,
    Right,
    Drop, // Normal Falling
    SoftDrop, // Player Fall
    HardDrop, // Tp to Bottom
    RotateLeft,
    RotateRight,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum GameMode {
    #[default]
    Normal,
    Hyper,
    Swap,
}

pub fn serialize_message<T: Serialize>(msg: T) -> Vec<u8> {
    let mut buf = bincode::serialize(&msg).expect("Failed serializing message");
    buf.insert(0, buf.len() as u8);
    buf
}
