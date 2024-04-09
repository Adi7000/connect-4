use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Game {
    pub name: String,
}

#[derive(Properties, PartialEq)]
pub struct State {
    pub games: Vec<Game>,
    pub selected_game: Game,
    pub on_click: Callback<Game>,
}

#[function_component(GamesList)]
pub fn games_list(
    State {
        games,
        selected_game,
        on_click,
    }: &State,
) -> Html {
    let on_click = on_click.clone();
    let game_options_html = games
        .iter()
        .map(|game| {
            let on_game_select = {
                let on_click = on_click.clone();
                let game = game.clone();
                Callback::from(move |_| on_click.emit(game.clone()))
            };
            if game == selected_game {
                html! {
                    <b onclick={on_game_select} id="game_option">{format!("{}", game.name)}</b>
                }
            } else {
                html! {
                    <p onclick={on_game_select} id="game_option">{format!("{}", game.name)}</p>
                }
            }   
        })
        .collect();

    game_options_html
}
