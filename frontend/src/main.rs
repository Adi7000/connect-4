mod app;
use app::App;
mod board;
mod menu_bar;

fn main() {
    yew::Renderer::<App>::new().render();
}
