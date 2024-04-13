mod app;
use app::App;
mod api;
mod board;
mod menu_bar;
mod connect_minimax;

fn main() {
    yew::Renderer::<App>::new().render();
}
