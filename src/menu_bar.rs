use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Game {
    pub name: String,
}
#[derive(Clone, PartialEq, Debug)]
pub enum Difficulty {
    Easy,
    Hard,
}

#[derive(Properties, PartialEq)]
pub struct MenuGameSelect {
    pub games: Vec<Game>,
    pub selected_game: Game,
    pub on_click: Callback<Game>,
}

#[derive(Properties, PartialEq)]
pub struct MenuDifficultySelect {
    pub difficulties: Vec<Difficulty>,
    pub selected_difficulty: Difficulty,
    pub on_click: Callback<Difficulty>,
}

#[function_component(GamesList)]
pub fn games_list(
    MenuGameSelect {
        games,
        selected_game,
        on_click,
    }: &MenuGameSelect,
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

#[function_component(DifficultyList)]
pub fn difficulty_list(
    MenuDifficultySelect {
        difficulties,
        selected_difficulty,
        on_click,
    }: &MenuDifficultySelect,
) -> Html {
    let on_click = on_click.clone();
    let difficulty_options_html = difficulties
        .iter()
        .map(|difficulty| {
            let on_difficulty_select = {
                let on_click = on_click.clone();
                let difficulty = difficulty.clone();
                Callback::from(move |_| on_click.emit(difficulty.clone()))
            };
            if difficulty == selected_difficulty {
                html! {
                    <b onclick={on_difficulty_select} id="difficulty_option">{format!("{:?}", difficulty)}</b>
                }
            } else {
                html! {
                    <p onclick={on_difficulty_select} id="difficulty_option">{format!("{:?}", difficulty)}</p>
                }
            }   
        })
        .collect();

    difficulty_options_html
}
