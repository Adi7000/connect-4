use crate::models::GameState;
use crate::minimax::drop_piece;
use crate::minimax::minimax;
use crate::minimax::get_next_open_row;
use crate::minimax::is_valid_location;

use rocket::serde::json::Json;
use reqwest::{Client};
use rocket::response::content;
// use rocket::serde::json::Value;
// use rocket::http::Status;


const PORT_NUMBER: u16 = 5000;
const AI_PIECE: i32 = 2;
const PLAYER_PIECE: i32 = 1;


#[get("/")]
pub async fn index() -> String {
    String::from("Hello World")
}

#[get("/connect", format= "json", data = "<game_state>")]
pub fn get_connect_move(game_state: Json<GameState>) -> Json<GameState> {

    // EXTRACT THE BOARD FROM THE REQUEST
    let extracted_game_state: GameState = game_state.clone().into_inner();
    let mut board = extracted_game_state.board_state.clone(); 

    // ENSURE THE GAME STATE IS FOR CONNECT 4
    if !extracted_game_state.connect_4 {
        let error_game_state: GameState = GameState {
            connect_4: extracted_game_state.connect_4.clone(),
            board_state: board.clone(),
            difficulty: extracted_game_state.difficulty.clone(),
            error: -1,
        };
        return Json(error_game_state);
    }

    // PRINT BOARD FOR SANITY CHECK
    println!("BEFORE AI MOVE:");
    for row in &board {
        println!("{:?}", row);
    }

    // SET DEPTH ACCORDING TO DIFFICULTY
    let depth = if extracted_game_state.difficulty == 2 { 5 } else{ 3 };

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
            connect_4: extracted_game_state.connect_4.clone(),
            board_state: board.clone(),
            difficulty: extracted_game_state.difficulty.clone(),
            error: 0,
        };

        return Json(res);
    }

    game_state
}

#[get("/otto")]
pub fn get_otto_move() -> String {
    String::from("otto response")
}