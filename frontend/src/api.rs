use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const PORT_NUMBER: u16 = 5000;
const EMPTY: i32 = 0;
const AI_PIECE: i32 = 2;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub connect_4: bool,            // TRUE FOR CONNECT 4
    pub board_state: Vec<Vec<i32>>, // BOARD STATE
    pub difficulty: u32,            // 1 FOR EASY; 2 FOR HARD
    pub error: i32, // -1 indicates error // CHECK THIS FIELD BEFORE USING API RESPONSE
}


use crate::connect_minimax::drop_piece;
use crate::connect_minimax::minimax;
use crate::connect_minimax::get_next_open_row;
use crate::connect_minimax::is_valid_location;

pub fn get_connect_move(game_state: &GameState) -> GameState {

    println!("{:?}", game_state);
    let mut board = game_state.board_state.clone(); 

    // PRINT BOARD FOR SANITY CHECK
    println!("BEFORE AI MOVE:");
    for row in &board {
        println!("{:?}", row);
    }

    // SET DEPTH ACCORDING TO DIFFICULTY
    let depth = if game_state.difficulty == 2 { 5 } else{ 3 };

    if let (column, score) = minimax(&mut board, depth, -1000, 1000, true) {
        // PUT AI PIECE IN BEST LOCATION
        let col = column.unwrap();
        if is_valid_location(&mut board, col) {
            let row = get_next_open_row(&mut board, col);
            drop_piece(&mut board, row, col, AI_PIECE)
        }
    
        // PRINT UPDATED BOARD FOR SANITY CHECK
        println!("AFTER AI MOVE:");
        println!("Move Column: {:?}, Score: {}", col, score);
        for row in &board {
            println!("{:?}", row);
        }

        // MAKE THE UPDATED GameState
        let res: GameState = GameState{
            connect_4: game_state.connect_4.clone(),
            board_state: board.clone(),
            difficulty: game_state.difficulty.clone(),
            error: 0,
        };

        return res.clone();
    }

    game_state.clone()
}


use log::info;
use web_sys::console;
use console_error_panic_hook; // Add this import


pub fn get_connect4_computer_move(
    game_state: &mut GameState,
) -> GameState {

    // Print debug message to browser console
    console::log_1(&format!("{:?}", game_state).into());

    get_connect_move(game_state).clone() // RETURN BOARD WITH COPUTER MOVE
    // WILL BE HELPFUL TO PRINT THIS AND SEE
    
    // game_state.clone()

    // let url = format!("http://localhost:{PORT_NUMBER}/project/connect4");
    // let resp = match Request::get(&url).send().await {
    //     Ok(resp) => resp,
    //     Err(err) => return Err(err),
    // };
    // Ok(resp)
}
