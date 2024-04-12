use crate::api::{get_connect4_computer_move, GameState};
use yew::{function_component, html, use_state, Callback, Html, Properties};
#[derive(Properties, Clone, PartialEq)]
// use this to decide between connect4 and toot board
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

#[derive(Clone, PartialEq, Copy)]
pub enum Color {
    Empty,
    Red,
    Yellow,
}

#[function_component(Board)]
pub fn board(_props: &BoardProps) -> Html {
    let mut game_state = GameState {
        connect_4: true,
        board_state: vec![vec![0; 6]; 7],
        difficulty: 1,
        error: 0,
    };

    // wasm_bindgen_futures::spawn_local(async move {
    //     game_state = computer_move(game_state).await;
    // });

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
            set_cell_colors.set(new_cell_colors);
            current_player.set(match *current_player {
                Color::Red => Color::Yellow,
                Color::Yellow => Color::Red,
                _ => unreachable!(),
            });
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

// async fn computer_move(game_state: GameState) -> GameState {
//     let next_game_state = match get_connect4_computer_move(game_state.clone()).await {
//         Ok(value) => match serde_json::from_value::<GameState>(value) {
//             Ok(parsed) => parsed,
//             Err(err) => {
//                 println!("Error parsing JSON: {:?}", err);
//                 return game_state;
//             }
//         },
//         Err(err) => {
//             // Handle the error if needed
//             println!("Error: {:?}", err);
//             return game_state;
//         }
//     };
//     next_game_state
// }
