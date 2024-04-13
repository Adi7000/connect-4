use crate::board::*;
use crate::nav_bar::NavBar;
use yew::prelude::*;
#[function_component(TootOttoGame)]
pub fn toot_otto_game() -> Html {
    html! {
        <main>
            <NavBar />
            <h1>{"Toot & Otto"}</h1>
            <Board game_type={GameType::TootOtto}/>
        </main>
    }
}
