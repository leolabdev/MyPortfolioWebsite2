// npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch
// trunk serve
use yew::ServerRenderer;
mod app;
pub mod components;
pub mod models;
pub mod services;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}


// / we use `flavor = "current_thread"` so this snippet can be tested in CI,
// where tests are run in a WASM environment. You likely want to use
// the (default) `multi_thread` favor as:
// #[tokio::main]
// #[tokio::main(flavor = "current_thread")]
// async fn main() {
//     let renderer = ServerRenderer::<App>::new();
//
//     let rendered = renderer.render().await;
//
//     // Prints: <div>Hello, World!</div>
//     println!("{}", rendered);
// }
