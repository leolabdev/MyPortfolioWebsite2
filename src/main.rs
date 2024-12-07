// // fn main() {
// //     println!("Hello, world!");
// // }
//
// // use stylist::css;
// use yew::prelude::*;
// use stylist::{style, yew::styled_component};
//
// // #[function_component]'
// #[styled_component]
// fn App() -> Html {
//     let counter = use_state(|| 0);
//     let onclick = {
//         let counter = counter.clone();
//         move |_| {
//             let value = *counter + 1;
//             counter.set(value);
//         }
//     };
//     // html! {
//     //     <div>
//     //         <button class={css!("color: red;")} {onclick}>{ "+2" }</button>
//     //         <p>{ *counter }</p>
//     //          <button>{ "+2" }</button>
//     //     </div>
//     // }
//     html! {<div class={css!("color: red;")}>{"Hello World!"}</div>}
//
//
// }
//
// fn main() {
//     yew::Renderer::<App>::new().render();
// }

// use yew::Renderer;
// use MyPortfolioWebsite2::App;
//
// fn main() {
//     Renderer::<App>::new().render();
// }

mod app;
use app::App;



fn main() {
    yew::Renderer::<App>::new().render();
}