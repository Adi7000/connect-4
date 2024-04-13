use crate::api::{get_connect4_computer_move, GameState};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::{function_component, html, use_state, Callback, Html, Properties};
#[derive(Properties, Clone, PartialEq)]
pub struct BoardProps {
    pub game_type: GameType,
}
#[derive(Clone, PartialEq, Copy)]
pub enum GameType {
    Connect4,
    TootOtto,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ColProps {
    col_index: usize,
    #[prop_or_default]
    cell_letters: Vec<Letter>,
    #[prop_or_default]
    cell_colors: Vec<Color>,
    game_type: GameType,
    pub on_click: Callback<usize>,
}
#[derive(Properties, Clone, PartialEq)]
pub struct Connect4CellProps {
    color: Color,
    x: usize,
    y: usize,
}

#[derive(Properties, Clone, PartialEq)]
pub struct TootOttoCellProps {
    letter: Letter,
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Copy)]
pub enum Letter {
    Empty,
    T,
    O,
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Color {
    Empty,
    Red,
    Yellow,
}

fn color_to_int(color: &Color) -> i32 {
    match color {
        Color::Empty => 0,
        Color::Red => 1,
        Color::Yellow => 2,
        // Add other colors if needed
    }
}

pub fn transpose(input: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut output = vec![vec![0; input.len()]; input[0].len()];

    for (i, row) in input.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            output[j][i] = val;
        }
    }
    output
}

pub fn check_for_win(board_state: Vec<Vec<Color>>) -> Option<Color> {
    let mut consecutive_count: i32;
    let mut current_color: Color;

    //check vertical
    for column in &board_state {
        //handle first item in column
        consecutive_count = 1;
        current_color = column[0];
        for cell in &column[1..] {
            if *cell != Color::Empty {
                if current_color == *cell {
                    //handle next item is same color
                    consecutive_count += 1;
                } else {
                    //handle next item is different color
                    consecutive_count = 1;
                    current_color = *cell
                }
                //check 4 consecutives
                if consecutive_count == 4 {
                    return Some(current_color);
                }
            } else {
                //skip if empty
                consecutive_count = 0;
                current_color = Color::Empty;
            }
        }
    }

    //check horizontal win
    for row_id in 0..6 {
        //handle first item in row
        consecutive_count = 1;
        current_color = board_state[0][row_id];
        for col_id in 1..7 {
            let cell = board_state[col_id][row_id];
            if cell != Color::Empty {
                if current_color == cell {
                    //handle next item is same color
                    consecutive_count += 1;
                } else {
                    //handle next item is different color
                    consecutive_count = 1;
                    current_color = cell
                }
                //check 4 consecutives
                if consecutive_count == 4 {
                    return Some(current_color);
                }
            } else {
                //skip if empty
                consecutive_count = 0;
                current_color = Color::Empty;
            }
        }
    }

    let max_row: i32 = 6;
    let max_col: i32 = 7;

    //check diagonal win (bottom left to top right)
    for diagonal_id in 0..(max_row + max_col - 1) {
        //similar to row_id
        //handle first item in diagonal
        consecutive_count = 1;
        current_color = if diagonal_id < max_row {
            board_state[0][diagonal_id as usize]
        } else {
            Color::Empty
        };
        let mut col_id = 1;
        let mut row_id = diagonal_id - 1;
        loop {
            if row_id < 0 || col_id >= max_col {
                break;
            };
            let cell = if row_id >= max_row {
                Color::Empty
            } else {
                board_state[col_id as usize][row_id as usize]
            };

            if cell != Color::Empty {
                if current_color == cell {
                    //handle next item is same color
                    consecutive_count += 1;
                } else {
                    //handle next item is different color
                    consecutive_count = 1;
                    current_color = cell;
                }
                //check 4 consecutives
                if consecutive_count == 4 {
                    return Some(current_color);
                }
            } else {
                //skip if empty
                consecutive_count = 0;
                current_color = Color::Empty;
            }
            col_id += 1;
            row_id -= 1;
        }
    }

    //check diagonal win 2 (top left to bottom right)
    for diagonal_id in 0..(max_row + max_col - 1) {
        //similar to row_id
        //handle first item in diagonal
        consecutive_count = 1;
        current_color = if diagonal_id < max_row {
            board_state[(max_col - 1) as usize][diagonal_id as usize]
        } else {
            Color::Empty
        };
        let mut col_id = max_col - 2;
        let mut row_id = diagonal_id - 1;
        loop {
            if row_id < 0 || col_id < 0 {
                break;
            };
            let cell = if row_id >= max_row {
                Color::Empty
            } else {
                board_state[col_id as usize][row_id as usize]
            };

            if cell != Color::Empty {
                if current_color == cell {
                    //handle next item is same color
                    consecutive_count += 1;
                } else {
                    //handle next item is different color
                    consecutive_count = 1;
                    current_color = cell;
                }
                //check 4 consecutives
                if consecutive_count == 4 {
                    return Some(current_color);
                }
            } else {
                //skip if empty
                consecutive_count = 0;
                current_color = Color::Empty;
            }
            col_id -= 1;
            row_id -= 1;
        }
    }

    None
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let cell_colors = use_state(|| vec![vec![Color::Empty; 6]; 7]);
    let cell_letters = use_state(|| vec![vec![Letter::Empty; 6]; 7]);
    let cell_colors_clone = cell_colors.clone();
    let cell_letters_clone = cell_letters.clone();
    let current_player = use_state(|| Color::Red);
    let current_letter = use_state(|| Letter::T);

    let game_type = props.game_type.clone(); // Clone game_type here

    let on_column_click = {
        let set_cell_colors = cell_colors.clone();
        let set_cell_letters = cell_letters.clone();
        let current_player = current_player.clone();
        let current_letter = current_letter.clone();
        Callback::from(move |col_index: usize| match game_type {
            // Use the cloned game_type here
            GameType::Connect4 => {
                let mut new_cell_colors = cell_colors_clone.clone().to_vec();
                for cell_color in new_cell_colors[col_index].iter_mut().rev() {
                    if *cell_color == Color::Empty {
                        *cell_color = *current_player;
                        break;
                    }
                }

                let board_state: Vec<Vec<i32>> = new_cell_colors
                .iter()
                .map(|row| row.iter().map(|color| color_to_int(color)).collect())
                .collect();

                if let Some(winner) = check_for_win(new_cell_colors.clone()) {
                    //set all cell colors to winner
                    set_cell_colors.set(vec![vec![winner; 6]; 7]);
                }
                else {
                    set_cell_colors.set(new_cell_colors.clone());
                }

                let transpose_board_state = transpose(board_state);
            
                let mut game_state = GameState {
                    connect_4: true,
                    board_state: transpose_board_state.clone(),
                    difficulty: 1,
                    error: 0,
                };

                console::log_1(&format!("{:?}", game_state).into());

                set_cell_colors.set(new_cell_colors);

                // Simulate computer move
                let mut game_state_clone = game_state.clone();
                let set_cell_colors = set_cell_colors.clone();
                let new_game_state = computer_move(&mut game_state_clone);
                let x = transpose(new_game_state.board_state);
                let new_cell_colors: Vec<Vec<Color>> = x
                    .iter()
                    .enumerate()
                    .map(|(x, col)| {
                        col.iter()
                            .enumerate()
                            .map(|(y, &cell)| match cell {
                                0 => Color::Empty,
                                1 => Color::Red,
                                2 => Color::Yellow,
                                _ => unreachable!(),
                            })
                            .collect()
                    })
                    .collect();
                
                if let Some(winner) = check_for_win(new_cell_colors.clone()) {
                    //set all cell colors to winner
                    set_cell_colors.set(vec![vec![winner; 6]; 7]);
                }
                else {
                    set_cell_colors.set(new_cell_colors.clone());
                }
                
            }

    
            GameType::TootOtto => {
                let mut new_cell_letters = cell_letters_clone.clone().to_vec();
                for cell_letter in new_cell_letters[col_index].iter_mut().rev() {
                    if *cell_letter == Letter::Empty {
                        *cell_letter = *current_letter;
                        break;
                    }
                }
                set_cell_letters.set(new_cell_letters);
                current_letter.set(match *current_letter {
                    Letter::T => Letter::O,
                    Letter::O => Letter::T,
                    _ => unreachable!(),
                });
            }
        })
    };
    match props.game_type {
        GameType::Connect4 => {
            html! {
                <board>
                    {for (0..7).map(|col_index| {
                        html! {
                            <Column cell_colors={cell_colors[col_index].clone()} col_index={col_index} on_click={on_column_click.clone()} game_type={props.game_type} />
                        }
                    })}
                </board>
            }
        }
        GameType::TootOtto => {
            html! {
                <board>
                    {for (0..7).map(|col_index| {
                        html! {
                            <Column cell_letters={cell_letters[col_index].clone()} col_index={col_index} on_click={on_column_click.clone()} game_type={props.game_type} />
                        }
                    })}
                </board>
            }
        }
    }
}

#[function_component(Column)]
pub fn column(props: &ColProps) -> Html {
    let on_click = props.on_click.clone();
    let col_index = props.col_index.clone();
    let on_column_click = {
        let on_click = on_click.clone();
        Callback::from(move |_| on_click.emit(col_index))
    };
    match props.game_type {
        GameType::Connect4 => {
            html! {
                <column onclick={on_column_click}>
                    {for (0..6).map(|row_index| {
                        html! {
                            <Connect4Cell color={props.cell_colors[row_index].clone()} x={row_index} y={props.col_index} />
                        }
                    })}
                </column>
            }
        }
        GameType::TootOtto => {
            html! {
                <column onclick={on_column_click}>
                    {for (0..6).map(|row_index| {
                        html! {
                            <TootOttoCell letter={props.cell_letters[row_index].clone()} x={row_index} y={props.col_index} />
                        }
                    })}
                </column>
            }
        }
    }
}

#[function_component(Connect4Cell)]
pub fn cell(props: &Connect4CellProps) -> Html {
    let cell_id = format!("c{},{}", props.x, props.y);
    let cell_color = match props.color {
        Color::Red => "red",
        Color::Yellow => "yellow",
        Color::Empty => "white",
    };
    html! {
        <cell id={cell_id} style={format!("background-color:{}", cell_color)}></cell>
    }
}




fn computer_move(game_state: &mut GameState) -> GameState {

    let resp = get_connect4_computer_move(game_state);

    return resp.clone();
    }

#[function_component(TootOttoCell)]
pub fn cell(props: &TootOttoCellProps) -> Html {
    let cell_id = format!("c{},{}", props.x, props.y);
    let cell_letter;
    let cell_color;
    match props.letter {
        Letter::T => {
            cell_letter = "T";
            cell_color = "#eff16e";
        }
        Letter::O => {
            cell_letter = "O";
            cell_color = "#a4feec";
        }
        Letter::Empty => {
            cell_letter = "";
            cell_color = "white";
        }
    };
    html! {
        <cell id={cell_id} style={format!("background-color:{}", cell_color)}>{cell_letter}</cell>
    }
}
