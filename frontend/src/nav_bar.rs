use crate::router::{Connect4Route, MainRoute, TootOttoRoute};
use yew::prelude::*;
use yew_router::hooks::use_navigator;
#[function_component(NavBar)]
pub fn navbar() -> Html {
    let navigator = use_navigator().unwrap();

    let go_home_button = {
        let navigator = navigator.clone();
        let on_click = Callback::from(move |_| navigator.push(&MainRoute::Home));
        html! {
            <button onclick={on_click}>{"Home"}</button>
        }
    };
    let go_connect4_rules_button = {
        let navigator = navigator.clone();
        let on_click = Callback::from(move |_| navigator.push(&Connect4Route::Rules));
        html! {
            <button onclick={on_click}>{"Connect4 Rules"}</button>
        }
    };

    let go_toototto_rules_button = {
        let navigator = navigator.clone();
        let on_click = Callback::from(move |_| navigator.push(&TootOttoRoute::Rules));
        html! {
            <button onclick={on_click}>{"Toot & Otto Rules"}</button>
        }
    };

    let go_connect4_game_button = {
        let navigator = navigator.clone();
        let on_click = Callback::from(move |_| navigator.push(&Connect4Route::Game));
        html! {
            <button onclick={on_click}>{"Play Connect4"}</button>
        }
    };

    let go_toototto_game_button = {
        let navigator = navigator.clone();
        let on_click = Callback::from(move |_| navigator.push(&TootOttoRoute::Game));
        html! {
            <button onclick={on_click}>{"Play Toot & Otto"}</button>
        }
    };

    html! {
            <nav_bar>
                {go_home_button}
                {go_connect4_rules_button}
                {go_connect4_game_button}
                {go_toototto_rules_button}
                {go_toototto_game_button}
            </nav_bar>
    }
}
