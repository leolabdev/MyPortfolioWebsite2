// npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch
// trunk serve

mod app;
pub mod components;
pub mod models;
pub mod services;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
