/*
 * File: filename.rs
 * Author: Fahrin Bushra
 * Date: April 12, 2024
 * Adapted From: https://github.com/KeithGalli/Connect4-Python/tree/master
 * Description: Minimax Algorithm for Connect4 AI
*/

use rand::seq::SliceRandom;
use std::cmp::{max, min};


// CONSTANTS
const PORT_NUMBER: u16 = 5000;
const AI_PIECE: i32 = 2;
const PLAYER_PIECE: i32 = 1;
const WINDOW_LENGTH: usize = 4;
const EMPTY: i32 = 0;

fn drop_piece(board: &mut Vec<Vec<i32>>, row: usize, col: usize, player: i32) {
    board[row][col] = player;
}

fn is_valid_location(board: &mut Vec<Vec<i32>>, col: usize) -> bool{
	board[0][col] == 0
}


fn get_valid_locations(board: &mut Vec<Vec<i32>>) -> Vec<usize>{
	let mut valid_locations: Vec<usize> = vec![];
    let col_count: usize = board[0].len();

	for col in 0..col_count{
		if is_valid_location(board, col){
            valid_locations.push(col.try_into().unwrap());
        }
    }
	return valid_locations
}

fn get_next_open_row(board: &mut Vec<Vec<i32>>, col: usize) -> usize{
    let row_count = board.len() - 1;
    for r in (0..=row_count).rev() {
        if board[r][col] == 0{
            return r;
        }
    }     
    return 30;
}

fn winning_move(board: &Vec<Vec<i32>>, piece: i32) -> bool{
    // Check horizontal locations for win
    let COLUMN_COUNT = board[0].len();
    let ROW_COUNT = board.len();

    for c in 0..COLUMN_COUNT - 3 {
        for r in 0..ROW_COUNT {
            if board[r][c] == piece && board[r][c + 1] == piece && board[r][c + 2] == piece && board[r][c + 3] == piece{
                return true;
            }
        }
    }
    
    // Check vertical locations for win
    for c in 0..COLUMN_COUNT {
        for r in 0..ROW_COUNT - 3 {
            if board[r][c] == piece && board[r + 1][c] == piece && board[r + 2][c] == piece && board[r + 3][c] == piece{
                return true;
            }
        }
    }
    
    // Check positively sloped diagonals
    for c in 0..COLUMN_COUNT - 3 {
        for r in 0..ROW_COUNT - 3 {
            if board[r][c] == piece && board[r + 1][c + 1] == piece && board[r + 2][c + 2] == piece && board[r + 3][c + 3] == piece {
                return true;
            }
        }
    }
    
    // Check negatively sloped diagonals
    for c in 0..COLUMN_COUNT - 3 {
        for r in 3..ROW_COUNT {
            if board[r][c] == piece && board[r - 1][c + 1] == piece && board[r - 2][c + 2] == piece && board[r - 3][c + 3] == piece{
                return true;
            }
        }
    }
    false
}

fn is_terminal_node(board: &mut Vec<Vec<i32>>) -> bool {
    let valid_locs: Vec<usize> = get_valid_locations(board);
    return winning_move(board, PLAYER_PIECE) || winning_move(board, AI_PIECE) || valid_locs.len() == 0;
}


fn evaluate_window(window: &[i32], piece: i32) -> i32 {
    let mut score = 0;
    let opp_piece = if piece == PLAYER_PIECE { AI_PIECE } else { PLAYER_PIECE };

    if window.iter().filter(|&&val| val == piece).count() == 4 {
        score += 100;
    } else if window.iter().filter(|&&val| val == piece).count() == 3 && window.iter().filter(|&&val| val == EMPTY).count() == 1{
        score += 5;
    } else if window.iter().filter(|&&val| val == piece).count() == 2 && window.iter().filter(|&&val| val == EMPTY).count() == 2{
        score += 2;
    }
    if window.iter().filter(|&&val| val == opp_piece).count() == 3 && window.iter().filter(|&&val| val == EMPTY).count() == 1{
        score -= 4;
    }
    score
}

fn score_position(board: &Vec<Vec<i32>>, piece: i32) -> i32 {
    let mut score = 0;
    let COLUMN_COUNT = board[0].len();
    let ROW_COUNT = board.len();

    // Score center column
    let center_array: Vec<i32> = board.iter().map(|row| row[COLUMN_COUNT / 2]).collect();
    let center_count = center_array.iter().filter(|&&x| x == piece).count();
    score += (center_count as i32) * 3;

    // Score Horizontal
    for row in board {
        for c in 0..=COLUMN_COUNT - WINDOW_LENGTH {
            let window: Vec<i32> = row[c..c + WINDOW_LENGTH].to_vec();
            score += evaluate_window(&window, piece);
        }
    }

    // Score Vertical
    for c in 0..COLUMN_COUNT {
        let col_array: Vec<i32> = (0..ROW_COUNT).map(|r| board[r][c]).collect();
        for r in 0..=ROW_COUNT - WINDOW_LENGTH {
            let window: Vec<i32> = col_array[r..r + WINDOW_LENGTH].to_vec();
            score += evaluate_window(&window, piece);
        }
    }

    // Score positive sloped diagonal
    for r in 0..=ROW_COUNT - WINDOW_LENGTH {
        for c in 0..=COLUMN_COUNT - WINDOW_LENGTH {
            let window: Vec<i32> = (0..WINDOW_LENGTH)
                .map(|i| board[r + i][c + i])
                .collect();
            score += evaluate_window(&window, piece);
        }
    }

    // Score negative sloped diagonal
    for r in 0..=ROW_COUNT - WINDOW_LENGTH {
        for c in (WINDOW_LENGTH - 1)..COLUMN_COUNT {
            let window: Vec<i32> = (0..WINDOW_LENGTH)
                .map(|i| board[r + WINDOW_LENGTH - 1 - i][c - i])
                .collect();
            score += evaluate_window(&window, piece);
        }
    }

    score
}


fn random_choice(items: &Vec<usize>) -> Option<&usize> {
    let mut rng = rand::thread_rng();
    items.choose(&mut rng)
}

fn minimax(board: &mut Vec<Vec<i32>>, depth: i32, alpha: i32, beta: i32, maximizing_player: bool) -> (Option<usize>, i32) {
    let valid_locations = get_valid_locations(board);
    let is_terminal = is_terminal_node(board);

    if depth == 0 || is_terminal {
        if is_terminal {
            if winning_move(board, AI_PIECE) {
                return (None, 1000000);
            } else if winning_move(board, PLAYER_PIECE) {
                return (None, -1000000);
            } else {
                return (None, 0);
            }
        } else {
            return (None, score_position(board, AI_PIECE));
        }
    }

    if maximizing_player {
        let mut value = i32::MIN;
        let mut column: usize = *random_choice(&valid_locations).unwrap();
        for &col in &valid_locations {
            let row = get_next_open_row(board, col);
            let mut b_copy = board.clone();
            drop_piece(&mut b_copy, row, col, AI_PIECE);
            let new_score = minimax(&mut b_copy, depth - 1, alpha, beta, false).1;
            if new_score > value {
                value = new_score;
                column = col;
            }
            let alpha = max(alpha, value);
            if alpha >= beta {
                break;
            }
        }
        return (Some(column), value);
    } else {
        let mut value = i32::MAX;
        let mut column: usize = *random_choice(&valid_locations).unwrap();
        for &col in &valid_locations {
            let row = get_next_open_row(board, col);
            let mut b_copy = board.clone();
            drop_piece(&mut b_copy, row, col, PLAYER_PIECE);
            let new_score = minimax(&mut b_copy, depth - 1, alpha, beta, true).1;
            if new_score < value {
                value = new_score;
                column = col;
            }
            let beta = min(beta, value);
            if alpha >= beta {
                break;
            }
        }
        return (Some(column), value);
    }
}