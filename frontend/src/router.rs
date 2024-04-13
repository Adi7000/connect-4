use crate::nav_bar::NavBar;
use crate::pages::connect4_game::Connect4Game;
use crate::pages::connect4_rules::Connect4Rules;
use crate::pages::home::Home;
use crate::pages::toot_otto_game::TootOttoGame;
use crate::pages::toot_otto_rules::TootOttoRules;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MainRoute {
    #[at("/")]
    Home,
    #[at("/connect4")]
    Connect4Root,
    #[at("/connect4/*")]
    Connect4,
    #[at("/toototto")]
    TootOttoRoot,
    #[at("/toototto/*")]
    TootOtto,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum Connect4Route {
    #[at("/connect4/rules")]
    Rules,
    #[at("/connect4/game")]
    Game,
    #[not_found]
    #[at("/connect4/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum TootOttoRoute {
    #[at("/toototto/rules")]
    Rules,
    #[at("/toototto/game")]
    Game,
    #[not_found]
    #[at("/toototto/404")]
    NotFound,
}

pub fn switch_main(route: MainRoute) -> Html {
    match route {
        MainRoute::Home => html! {<div><Home /></div>},
        MainRoute::Connect4Root | MainRoute::Connect4 => {
            html! {<Switch<Connect4Route> render={switch_connect4} />}
        }
        MainRoute::TootOttoRoot | MainRoute::TootOtto => {
            html! {<Switch<TootOttoRoute> render={switch_toototto} />}
        }
        MainRoute::NotFound => html! {<h1>{"404"}</h1>},
    }
}

pub fn switch_connect4(route: Connect4Route) -> Html {
    match route {
        Connect4Route::Rules => html! {<Connect4Rules />},
        Connect4Route::Game => html! {<Connect4Game />},
        Connect4Route::NotFound => html! {<h1>{"404"}</h1>},
    }
}

pub fn switch_toototto(route: TootOttoRoute) -> Html {
    match route {
        TootOttoRoute::Rules => html! {<TootOttoRules />},
        TootOttoRoute::Game => html! {<TootOttoGame />},
        TootOttoRoute::NotFound => html! {<h1>{"404"}</h1>},
    }
}
