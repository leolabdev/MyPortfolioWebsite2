use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <h1 class="text-3xl text-blue-600/100 font-bold underline">
        {"Hello World!"}
        </h1>
    }
}



// let counter = use_state(|| 0);
// let onclick = {
//     let counter = counter.clone();
//     move |_| {
//         let value = *counter + 1;
//         counter.set(value);
//     }
// };
//
// html! {
//     <div>
//         <button {onclick}>{ "+1" }</button>
//         <p>{ *counter }</p>
//     </div>
// }

// let style = use_style!("color: red;");