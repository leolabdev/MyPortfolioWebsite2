use yew::prelude::*;

#[function_component(Gradient)]
pub fn gradient() -> Html {
    html! {
        <div class="h-1 bg-gradient-to-r from-purple-500 to-blue-500"></div>
    }
}
