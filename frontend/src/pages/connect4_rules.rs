use crate::nav_bar::NavBar;
use yew::prelude::*;
#[function_component(Connect4Rules)]
pub fn connect4_rules() -> Html {
    html! {
        <main>
            <NavBar />
            <h1>{"Connect4 Rules"}</h1>
            <p>{"Players take turns inserting tokens.
                    Tokens can only be inserted to the top of the column.
                    Connect 4 of your color in a straight row  to win.
                    Rows can be horizontal, vertical, or diagonal."}
            </p>
        </main>
    }
}
