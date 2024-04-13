use crate::board::*;
use crate::menu_bar::Difficulty;
use crate::nav_bar::NavBar;
use yew::prelude::*;
#[function_component(Connect4Game)]
pub fn connect4_game() -> Html {
    html! {
        <main>
            <NavBar />
            <h1>{"Connect 4"}</h1>
            <Board game_type={GameType::Connect4} difficulty={Difficulty::Hard}/>
        </main>
    }
}
