// npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch
// trunk serve

mod app;
mod components;
use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
