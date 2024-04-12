/*
 * File: models.rs
 * Author: Fahrin Bushra
 * Date: April 11, 2024
 * Adapted From: https://www.youtube.com/watch?v=2vxvSMkm5Lg
 * Description: GAME STATE MODEL
*/
use rocket::serde::Deserialize;
use rocket::serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState{
    pub connect_4: bool, // TRUE FOR CONNECT 4
    pub board_state: Vec<Vec<i32>>, // BOARD STATE
    pub difficulty: u32, // 1 FOR EASY; 2 FOR HARD
    pub error: i32
}
