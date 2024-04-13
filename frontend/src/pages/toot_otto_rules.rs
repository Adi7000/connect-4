use crate::nav_bar::NavBar;
use yew::prelude::*;
#[function_component(TootOttoRules)]
pub fn toot_otto_rules() -> Html {
    html! {
        <main>
            <NavBar />
            <h1>{"Toot & Otto Rules"}</h1>
            <p>{"One player is TOOT and the other player is OTTO.
                Both players can place both T's and O's, based on their choice. 
                The first player who spells his or her winning combination - horizontally, vertically or diagonally - wins!"}
            </p>
        </main>
    }
}
