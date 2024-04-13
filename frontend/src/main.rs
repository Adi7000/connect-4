mod app;
use app::App;
mod board;
mod menu_bar;
mod nav_bar;
mod pages;
mod router;

fn main() {
    yew::Renderer::<App>::new().render();
}
