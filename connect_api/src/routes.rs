/*
 * File: routes.rs
 * Author: Fahrin Bushra
 * Date: April 11, 2024
 * Adapted From: https://www.youtube.com/watch?v=2vxvSMkm5Lg
 * Description: API ROUTES
*/


use crate::models::GameState;
use crate::helpers::*;
use crate::connect_minimax::drop_piece;
use crate::connect_minimax::minimax;
use crate::connect_minimax::get_next_open_row;
use crate::connect_minimax::is_valid_location;
use crate::otto_minimax::ai_move_easy;
use crate::chat_gpt::generate_text;

use rocket::serde::json::Json;
use reqwest::{Client};
use rocket::response::content;
use serde_json::json;

const PORT_NUMBER: u16 = 5000;

// CONNECT 4
const AI_PIECE: i32 = 2;
const PLAYER_PIECE: i32 = 1;

// TOOT OTTO
const T: i32 = 1;
const O: i32 = -1;


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


#[get("/otto", format= "json", data = "<game_state>")]
pub async fn get_otto_move(game_state: Json<GameState>) -> Json<GameState> {

    // EXTRACT THE BOARD FROM THE REQUEST
    let extracted_game_state: GameState = game_state.clone().into_inner();
    let mut board = extracted_game_state.board_state.clone(); 

    // ENSURE THE GAME STATE IS FOR OTTO TOOT
    if extracted_game_state.connect_4 {
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
    let depth = if extracted_game_state.difficulty == 1 { 5 } else{ -1 };

    if depth == -1 {
        let json_string = serde_json::to_string(&board).unwrap();
        let mut updated_board = generate_text(json_string.clone()).await;

        let diff_elem_count = count_different_elements(&updated_board, &board);
        let new_zero_count = count_zeros(&updated_board);
        let new_non_zero_count = count_non_zero_elements(&updated_board);
        let original_zero_count = count_zeros(&board);
        let original_non_zero_count = count_non_zero_elements(&board);

        if new_non_zero_count - original_non_zero_count != 1 || diff_elem_count != 1 || original_zero_count - new_zero_count != 1{
            println!("GPT FALTERED!");
            println!("diff_elem_count {}", diff_elem_count);
            println!("new_non_zero_count {}", new_non_zero_count);
            println!("original_non_zero_count {}", original_non_zero_count);
            updated_board = ai_move_easy(&mut board, 10);
        }

        println!("AFTER AI MOVE:");
        for row in &updated_board {
            println!("{:?}", row);
        }

        // MAKE THE UPDATED GameState
        let res: GameState = GameState{
            connect_4: extracted_game_state.connect_4.clone(),
            board_state: updated_board.clone(),
            difficulty: extracted_game_state.difficulty.clone(),
            error: 0,
        };

        return Json(res);
    }
    else {
        // UPDATE WITH AI MOVE
        let updated_board = ai_move_easy(&mut board, depth);
            
        // PRINT UPDATED BOARD FOR SANITY CHECK
        println!("AFTER AI MOVE:");
        for row in &updated_board {
            println!("{:?}", row);
        }

        // MAKE THE UPDATED GameState
        let res: GameState = GameState{
            connect_4: extracted_game_state.connect_4.clone(),
            board_state: updated_board.clone(),
            difficulty: extracted_game_state.difficulty.clone(),
            error: 0,
        };

        return Json(res);
    }
}