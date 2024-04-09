use yew::prelude::*;
mod menu_bar;
use menu_bar::*;

#[function_component(App)]
fn app() -> Html {
    // ---------------Menu Bar----------------
    let games = vec![
        Game {
            name: "Connect-4".to_string(),
        },
        Game {
            name: "OTTO".to_string(),
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

    html! {
    <>
        <div>
        <GamesList games={games}
        selected_game = {(*selected_game).clone()}
        on_click={on_game_select.clone()} />
        </div>
        <div>
        <DifficultyList difficulties={difficulties}
        selected_difficulty = {(*selected_difficulty).clone()}
        on_click={on_difficulty_select.clone()} />
        </div>
    </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
