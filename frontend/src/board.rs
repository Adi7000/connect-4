use yew::{function_component, html, use_state, Callback, Html, Properties};
#[derive(Properties, Clone, PartialEq)]
pub struct BoardProps {}

#[derive(Properties, Clone, PartialEq)]
pub struct ColProps {
    col_index: usize,
    #[prop_or(Color::Empty)]
    color: Color,
    pub on_click: Callback<Cell>,
}
#[derive(Properties, Clone, PartialEq)]
pub struct CellProps {
    color: Color,
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq)]
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
    let on_click = {
        let on_click = use_state(|| Callback::noop());
        let on_click = on_click.clone();
        Callback::from(move |cell: Cell| {
            on_click.emit(cell);
        })
    };

    html! {
        <board>
            {for (0..7).map(|col_index| {
                html! {
                    <Column col_index={col_index} on_click={on_click.clone()} />
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
                html! {
                    <Cell color={props.color.clone()} x={row_index} y={props.col_index} />
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
