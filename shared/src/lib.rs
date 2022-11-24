use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    GameStart, // Start the Game
    OtherTetrisMove(TetrisMove), // Other player peice moved
    GameState(GameState), // Set game state
    GameEnd(u8), // End the game | u8: Winning Player
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    TetrisMove(TetrisMove),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TetrisMove {
    Left, // Move Left
    Right, // Move Right
    Drop, // Normal Falling
    SoftDrop, // Player Fall
    HardDrop, // Tp to Bottom
    Rotate(bool), // Rotate | bool: Direction
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum GameState {
    #[default]
    Normal,
    Hyper,
    Swap,
}
