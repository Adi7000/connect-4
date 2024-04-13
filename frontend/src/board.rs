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

#[derive(Clone, PartialEq, Copy)]
pub enum Color {
    Empty,
    Red,
    Yellow,
}

pub enum Msg {
    ColumnClicked(usize),
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
                set_cell_colors.set(new_cell_colors);
                current_player.set(match *current_player {
                    Color::Red => Color::Yellow,
                    Color::Yellow => Color::Red,
                    _ => unreachable!(),
                });
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
