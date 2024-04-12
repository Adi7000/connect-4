use rand::Rng;
use rand::seq::SliceRandom;

const PLAYER_1: i32 = 1;
const PLAYER_2: i32 = -1;
const EMPTY: i32 = 0;
const T: i32 = 1;
const O: i32 = -1;


fn check_winner(state: &Vec<Vec<i32>>) -> (i32, i32) {

    let ROWS: usize = state.len();
    let COLS: usize = state[0].len();

    for i in 0..ROWS {
        for j in 0..COLS {
            // Check horizontal
            if j + 3 < COLS {
                if state[i][j] == T && state[i][j + 1] == O && state[i][j + 2] == O && state[i][j + 3] == T {
                    return (1, 0); // Player 1 wins
                }
                if state[i][j] == O && state[i][j + 1] == T && state[i][j + 2] == T && state[i][j + 3] == O {
                    return (-1, 0); // Player 2 (AI) wins
                }
            }
            // Check vertical
            if i + 3 < ROWS {
                if state[i][j] == T && state[i + 1][j] == O && state[i + 2][j] == O && state[i + 3][j] == T {
                    return (1, 0); // Player 1 wins
                }
                if state[i][j] == O && state[i + 1][j] == T && state[i + 2][j] == T && state[i + 3][j] == O {
                    return (-1, 0); // Player 2 (AI) wins
                }
            }
            // Check diagonal (top-left to bottom-right)
            if i + 3 < ROWS && j + 3 < COLS {
                if state[i][j] == T && state[i + 1][j + 1] == O && state[i + 2][j + 2] == O && state[i + 3][j + 3] == T {
                    return (1, 0); // Player 1 wins
                }
                if state[i][j] == O && state[i + 1][j + 1] == T && state[i + 2][j + 2] == T && state[i + 3][j + 3] == O {
                    return (-1, 0); // Player 2 (AI) wins
                }
            }
            // Check diagonal (bottom-left to top-right)
            if i >= 3 && j + 3 < COLS {
                if state[i][j] == T && state[i - 1][j + 1] == O && state[i - 2][j + 2] == O && state[i - 3][j + 3] == T {
                    return (1, 0); // Player 1 wins
                }
                if state[i][j] == O && state[i - 1][j + 1] == T && state[i - 2][j + 2] == T && state[i - 3][j + 3] == O {
                    return (-1, 0); // Player 2 (AI) wins
                }
            }
        }
    }
    (0, 0) // No winner yet
}

fn check_state(state: &Vec<Vec<i32>>) -> (i32, i32) {
    let ROWS: usize = state.len();
    let COLS: usize = state[0].len();

    let mut win_val = 0;
    let mut chain_val = 0;
    let mut temp_r;
    let mut temp_b;
    let mut temp_br;
    let mut temp_tr;
    for i in 0..ROWS {
        for j in 0..COLS {
            temp_r = 0;
            temp_b = 0;
            temp_br = 0;
            temp_tr = 0;
            for k in 0..=3 {
                if j + k < COLS {
                    temp_r += state[i][j + k];
                }
                if i + k < ROWS {
                    temp_b += state[i + k][j];
                }
                if i + k < ROWS && j + k < COLS {
                    temp_br += state[i + k][j + k];
                }
                if i as i32 - k as i32 >= 0 && j + k < COLS {
                    temp_tr += state[i - k][j + k];
                }
            }
            chain_val += temp_r * temp_r * temp_r;
            chain_val += temp_b * temp_b * temp_b;
            chain_val += temp_br * temp_br * temp_br;
            chain_val += temp_tr * temp_tr * temp_tr;

            if temp_r.abs() == 4 {
                win_val = temp_r;
            } else if temp_b.abs() == 4 {
                win_val = temp_b;
            } else if temp_br.abs() == 4 {
                win_val = temp_br;
            } else if temp_tr.abs() == 4 {
                win_val = temp_tr;
            }
        }
    }
    (win_val, chain_val)
}


fn value(state: &Vec<Vec<i32>>, depth: i32, ai_move_value: i32, alpha: i32, beta: i32) -> (i32, i32) {
    let ROWS: usize = state.len();
    let COLS: usize = state[0].len();

    let (win_val, chain_val) = check_state(&state.clone());
    if depth >= 4 {
        let mut ret_value = 0;
        let chain_val = chain_val * ai_move_value;
        ret_value = chain_val;
        if win_val == 4 * ai_move_value {
            ret_value = 99999;
        } else if win_val == 4 * ai_move_value * -1 {
            ret_value = 99999 * -1;
        }
        ret_value -= depth * depth;
        return (ret_value, -1);
    }

    if win_val == 4 * ai_move_value {
        return (99999 - depth * depth, -1);
    }
    if win_val == 4 * ai_move_value * -1 {
        return (99999 * -1 - depth * depth, -1);
    }

    if depth % 2 == 0 {
        // println!("AI depth 2 MOVEE board size {}", state.len());
        return min_state(&state.clone(), depth + 1, ai_move_value, alpha, beta);
    }
    max_state(&state.clone(), depth + 1, ai_move_value, alpha, beta)
}

fn max_state(state: &Vec<Vec<i32>>, depth: i32, ai_move_value: i32, alpha: i32, beta: i32) -> (i32, i32) {
    let ROWS: usize = state.len();
    let COLS: usize = state[0].len();

    let mut v = -100000007;
    let mut move_col = 300;
    let mut temp_val;
    let mut temp_state: Vec<Vec<i32>> = state.clone();
    let mut move_queue = Vec::new();
    for j in 0..COLS {
        let temp_state_success = fill_map(&mut temp_state, j, ai_move_value);
        if temp_state_success != -1 {
            // println!("AI MOVEE board size {}, success {}", state.len(), temp_state_success);
            temp_val = value(&temp_state.clone(), depth, ai_move_value, alpha, beta);
            if temp_val.0 == 99999 {
                return (99999, j.try_into().unwrap());
            }
            
            if temp_val.0 > v {
                v = temp_val.0;
                move_col = j;
                move_queue.clear();
                move_queue.push(j);
            } else if temp_val.0 == v {
                move_queue.push(j);
            }
            if v > beta {
                let move_choice = choose(&move_queue);
                return (v, move_choice);
            }
        }
    }
    let move_choice = choose(&move_queue);
    (v, move_choice)
}

fn min_state(state: & Vec<Vec<i32>>, depth: i32, ai_move_value: i32, alpha: i32, beta: i32) -> (i32, i32) {
    let ROWS: usize = state.len();
    let COLS: usize = state[0].len();
    
    let mut v = 100000007;
    let mut move_col = 300;
    let mut temp_val;
    let mut temp_state: Vec<Vec<i32>> = state.clone();
    let mut move_queue = Vec::new();
    for j in 0..COLS {
        
        let temp_state_success = fill_map(&mut temp_state, j, ai_move_value * -1);
        // println!("AI min state 2 MOVEE board size {}", state.len());
        if temp_state_success != -1 {
            // println!("AI min state 2 MOVEE board size {}", state.len());
            temp_val = value(&temp_state.clone(), depth, ai_move_value, alpha, beta);
            // println!("AI min state 2 MOVEE board size {}", state.len());
            if temp_val.0 == -99999 {
                return (-99999, j.try_into().unwrap());
            }
            if temp_val.0 < v {
                v = temp_val.0;
                move_col = j;
                move_queue.clear();
                move_queue.push(j);
            } else if temp_val.0 == v {
                move_queue.push(j);
            }
            if v < alpha {
                let move_choice = choose(&move_queue);
                return (v, move_choice);
            }
        }
    }
    let move_choice = choose(&move_queue);
    (v, move_choice)
}

pub fn choose(items: &Vec<usize>) -> i32 {
    let mut rng = rand::thread_rng();
    (*items.choose(&mut rng).unwrap_or(&0)).try_into().unwrap()
}

pub fn get_next_open_row(board: &mut Vec<Vec<i32>>, col: usize) -> usize{
    // println!("HERE {}", board.len());
    let row_count = board.len() - 1;
    for r in (0..=row_count).rev() {
        if board[r][col] == 0{
            return r;
        }
    }     
    return 30;
}


fn fill_map(state: &mut Vec<Vec<i32>>, col: usize, ai_move_value: i32)-> i32 {
    let row = get_next_open_row(state, col);
    if row != 30 {
        state[row][col] = ai_move_value;
        return 0;
    }

    return -1;
}


fn print_otto_board(state: &Vec<Vec<i32>>) {
    for row in state {
        for cell in row {
            let symbol = match *cell {
                T => 'T',
                O => 'O',
                _ => '-',
            };
            print!("{} ", symbol);
        }
        println!();
    }
}

pub fn drop_piece(board: &mut Vec<Vec<i32>>, row: usize, col: usize, token: i32) {
    board[row][col] = token;
}

pub fn ai_move_easy(state: &Vec<Vec<i32>>, depth: i32) -> Vec<Vec<i32>> {
    let x: Vec<usize> = vec![1, 0];
    let y = choose(&x);

    let ai_token: i32 = if y == 1 {-1} else {1};
    
    let rng = rand::thread_rng();
    let col = max_state(state, depth, ai_token, -10000007, 10000007).1 as usize;
    // return (ai_token, col);

    let mut board: Vec<Vec<i32>> = state.clone();
    fill_map(&mut board, col, ai_token);
    return board.clone()
}
