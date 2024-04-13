mod app;
use app::App;
mod api;
mod board;
mod menu_bar;
mod connect_minimax;
mod otto_minimax;

fn main() {
    yew::Renderer::<App>::new().render();
}
