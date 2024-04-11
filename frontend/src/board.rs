use yew::{function_component, html, use_state, Callback, Html, Properties};
#[derive(Properties, Clone, PartialEq)]
pub struct BoardProps {}

#[derive(Properties, Clone, PartialEq)]
pub struct ColProps {
    col_index: usize,
    #[prop_or(Color::Empty)]
    color: Color,
    pub on_click: Callback<usize>,
}

#[derive(Clone, PartialEq)]
pub enum Color {
    Red,
    Yellow,
    Empty,
}

pub enum Msg {
    ColumnClicked(usize),
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let selected_column = use_state(|| None);
    let on_column_select = {
        let selected_column = selected_column.clone();
        Callback::from(move |col_index| {
            selected_column.set(Some(col_index));
        })
    };

    html! {
        <board>
            {for (0..7).map(|col_index| {
                html! {
                    <Column col_index={col_index} on_click={on_column_select.clone()} />
                }
            })}
        </board>
    }
}

#[function_component(Column)]
pub fn column(props: &ColProps) -> Html {
    html! {
        <column>
            {for (0..6).map(|row_index| {
                let cell_id = format!("c{},{}", row_index, props.col_index);
                let cell_color = match props.color {
                    Color::Red => "red",
                    Color::Yellow => "yellow",
                    Color::Empty => "white",
                };
                html! {
                    <cell id={cell_id} style={format!("background-color: {}", cell_color)}></cell>
                }
            })}
        </column>
    }
}
