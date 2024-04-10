use yew::prelude::*;
mod menu_bar;
use menu_bar::*;

#[function_component(App)]
fn app() -> Html {
    // ---------------Menu Bar----------------
    let games = vec![
        Game {
            name: "Connect-4".to_string(),
            rules: "Players take turns inserting tokens.
                    Tokens can only be inserted to the top of the column.
                    Connect 4 of your color in a straight row  to win.
                    Rows can be horizontal, vertical, or diagonal.".to_string()
        },
        Game {
            name: "OTTO".to_string(),
            rules: "TODO".to_string()
        },
    ];
    let selected_game = use_state(|| games[0].clone()); //Default game
    let on_game_select = {
        let selected_game = selected_game.clone();
        Callback::from(move |game: Game| selected_game.set(game))
    };

    let difficulties = vec![Difficulty::Easy, Difficulty::Hard];
    let selected_difficulty = use_state(|| difficulties[0].clone()); //Default difficulty
    let on_difficulty_select = {
        let selected_difficulty = selected_difficulty.clone();
        Callback::from(move |difficulty: Difficulty| selected_difficulty.set(difficulty))
    };

    let rules_opened = use_state(|| false);
    let on_rules_select = {
        let rules_opened = rules_opened.clone();
        Callback::from(move |opened: bool| rules_opened.set(opened))
    };

    html! {
    <>
        <div style="padding:10px; text-align: center;">
            <div style="display:inline; margin:20px">
                <GamesList games={games}
                selected_game = {(*selected_game).clone()}
                on_click={on_game_select.clone()}/>
            </div>
            <div style="display:inline; margin:20px">
                <DifficultyList difficulties={difficulties}
                selected_difficulty = {(*selected_difficulty).clone()}
                on_click={on_difficulty_select.clone()} />
            </div>
            <div style="padding:10px;"> 
                <RulesButton selected_game = {(*selected_game).clone()}
                rules_opened={(*rules_opened).clone()}
                on_click={on_rules_select.clone()}/>
            </div>
        </div>
        
    </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
