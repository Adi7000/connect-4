use yew::{function_component, html, use_state, Callback, Html, Properties};
#[derive(Properties, Clone, PartialEq)]
pub struct BoardProps {}

#[derive(Properties, Clone, PartialEq)]
pub struct ColProps {
    col_index: usize,
    cell_colors: Vec<Color>,
    pub on_click: Callback<usize>,
}
#[derive(Properties, Clone, PartialEq)]
pub struct CellProps {
    color: Color,
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Color {
    Empty,
    Red,
    Yellow,
}

pub enum Msg {
    ColumnClicked(usize),
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
                if current_color == *cell { //handle next item is same color
                    consecutive_count += 1;
                } else { //handle next item is different color
                    consecutive_count = 1;
                    current_color = *cell
                }
                //check 4 consecutives
                if consecutive_count==4 {
                    return Some(current_color)
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
                if current_color == cell { //handle next item is same color
                    consecutive_count += 1;
                } else { //handle next item is different color
                    consecutive_count = 1;
                    current_color = cell
                }
                //check 4 consecutives
                if consecutive_count==4 {
                    return Some(current_color)
                }
            } else {
                //skip if empty
                consecutive_count = 0;
                current_color = Color::Empty;
            }
            
        }
    }

    let max_row:i32 = 6;
    let max_col:i32 = 7;

    //check diagonal win (bottom left to top right)
    for diagonal_id in 0..(max_row+max_col-1) { //similar to row_id
        //handle first item in diagonal
        consecutive_count = 1;
        current_color = if diagonal_id < max_row {
            board_state[0][diagonal_id as usize]
        } else {
            Color::Empty
        };
        let mut col_id=1;
        let mut row_id=diagonal_id-1;
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
                if current_color == cell { //handle next item is same color
                    consecutive_count += 1;
                } else { //handle next item is different color
                    consecutive_count = 1;
                    current_color = cell;
                }
                //check 4 consecutives
                if consecutive_count==4 {
                    return Some(current_color);
                }
            } else {
                //skip if empty
                consecutive_count = 0;
                current_color = Color::Empty;
            }
            col_id+=1;
            row_id-=1;
        }
    };

    //check diagonal win 2 (top left to bottom right)
    for diagonal_id in 0..(max_row+max_col-1) { //similar to row_id
        //handle first item in diagonal
        consecutive_count = 1;
        current_color = if diagonal_id < max_row {
            board_state[(max_col-1) as usize][diagonal_id as usize]
        } else {
            Color::Empty
        };
        let mut col_id=max_col - 2;
        let mut row_id=diagonal_id-1;
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
                if current_color == cell { //handle next item is same color
                    consecutive_count += 1;
                } else { //handle next item is different color
                    consecutive_count = 1;
                    current_color = cell;
                }
                //check 4 consecutives
                if consecutive_count==4 {
                    return Some(current_color);
                }
            } else {
                //skip if empty
                consecutive_count = 0;
                current_color = Color::Empty;
            }
            col_id-=1;
            row_id-=1;
        }
    }

    None
}




#[function_component(Board)]
pub fn board(_props: &BoardProps) -> Html {
    let cell_colors = use_state(|| vec![vec![Color::Empty; 6]; 7]);
    let cell_colors_clone = cell_colors.clone();
    let current_player = use_state(|| Color::Red);
    let on_column_click = {
        let set_cell_colors = cell_colors.clone();
        let current_player = current_player.clone();
        Callback::from(move |col_index: usize| {
            let mut new_cell_colors = cell_colors_clone.clone().to_vec();
            for cell_color in new_cell_colors[col_index].iter_mut().rev() {
                if *cell_color == Color::Empty {
                    *cell_color = *current_player;
                    break;
                }
            }
            set_cell_colors.set(new_cell_colors.clone());
            current_player.set(match *current_player {
                // Add these lines
                Color::Red => Color::Yellow,
                Color::Yellow => Color::Red,
                _ => unreachable!(),
            });
            //Check for win 
            if let Some(winner) = check_for_win(new_cell_colors.clone()) {
                //set all cell colors to winner
                set_cell_colors.set(vec![vec![winner; 6]; 7]);
            }
        })
    };

    html! {
        <board>
            {for (0..7).map(|col_index| {
                html! {
                    <Column cell_colors={cell_colors[col_index].clone()} col_index={col_index} on_click={on_column_click.clone()} />
                }
            })}
        </board>
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
    html! {
        <column onclick={on_column_click}>
            {for (0..6).map(|row_index| {
                html! {
                    <Cell color={props.cell_colors[row_index].clone()} x={row_index} y={props.col_index} />
                }
            })}
        </column>
    }
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
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
