use yew::prelude::*;

#[function_component(Intro)]
pub fn intro() -> Html {
    html! {
        <header id="intro" class="px-4 py-16 max-w-4xl mx-auto text-center">
            <p class="font-mono text-lg text-aqua">{"Hi, my name is"} <span class="font-sans text-4xl text-aqua">{"Leo Hannolainen"}</span></p>
            <h2 class="text-3xl font-light">{"I'm a web developer with a passion for building scalable web applications"}</h2>
            <p class="mt-4 text-lg leading-6">
                {"With a Bachelor's Degree in Software Engineering..."}
            </p>
        </header>
    }
}
