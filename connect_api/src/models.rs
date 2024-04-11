use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState{
    pub connect_4: bool, // TRUE FOR CONNECT 4
    pub board_state: Vec<Vec<i32>>, // BOARD STATE
    pub difficulty: u32, // 1 FOR EASY; 2 FOR HARD
    pub error: i32
}
