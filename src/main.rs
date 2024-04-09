use yew::prelude::*;
mod menu_bar;
use menu_bar::*;

#[function_component(App)]
fn app() -> Html {
    let games = vec![
        Game {
            name: "Connect-4".to_string()
        }, 
        Game {
            name: "OTTO".to_string()
        }
    ];

    let selected_game = use_state(||games[0].clone()); //Default game

    let on_game_select = {
        let selected_game = selected_game.clone();
        Callback::from(move |game: Game| selected_game.set(game))
    };

    html! {
    <>
        <div>
        <GamesList games={games}
        selected_game = {(*selected_game).clone()}
        on_click={on_game_select.clone()} />
        </div>
    </>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
