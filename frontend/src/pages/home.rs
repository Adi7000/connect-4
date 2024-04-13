use crate::nav_bar::NavBar;
use yew::prelude::*;
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <main>
        <NavBar />
            <h1>{"Rusty Connect4 + Toot and Otto"}</h1>
            <h2>{"Select an option"}</h2>
        </main>
    }
}
