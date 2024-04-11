use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Game {
    pub name: String,
    pub rules: String,
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

#[derive(Properties, PartialEq)]
pub struct MenuRulesButton {
    pub selected_game: Game,
    pub on_click: Callback<bool>,
    pub rules_opened: bool,
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
                    <u style="padding: 10px; display: inline;" onclick={on_game_select} id="game_option"> {format!("{}", game.name)}</u>
                }
            } else {
                html! {
                    <p style="padding: 10px; display: inline;" onclick={on_game_select} id="game_option">{format!("{}", game.name)}</p>
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
                    <u style="padding: 10px; display: inline;" onclick={on_difficulty_select} id="difficulty_option">{format!("{:?}", difficulty)}</u>
                }
            } else {
                html! {
                    <p style="padding: 10px; display: inline;" onclick={on_difficulty_select} id="difficulty_option">{format!("{:?}", difficulty)}</p>
                }
            }   
        })
        .collect();
    difficulty_options_html
    
}

#[function_component(RulesButton)]
pub fn rules_button(
    MenuRulesButton {
        selected_game,
        on_click,
        rules_opened,
    }: &MenuRulesButton,
) -> Html {
    let on_click = on_click.clone();

    if *rules_opened {
        let on_rules_select = Callback::from(move |_| on_click.emit(false));
        html! {
            <>
                <u style="padding: 10px; display: inline;" 
                onclick={on_rules_select} id="difficulty_option">{"Rules!"}</u>
                <p style="padding: 2px; width: 50ch; text-align: center; margin: 0 auto;">
                {selected_game.rules.clone()}
                </p>
            </>
        }
    } else {
        let on_rules_select = Callback::from(move |_| on_click.emit(true));
        html! {
            <p style="padding: 10px; display: inline;" 
            onclick={on_rules_select} id="difficulty_option">{"Rules!"}</p>
        }
    }
}