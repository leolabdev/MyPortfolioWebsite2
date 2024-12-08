use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="bg-plum text-center py-16 px-4 text-white">
            <h2 class="text-2xl mb-4">{"Contact me"}</h2>
            <p>{"I'm always interested in hearing about new studying or working opportunities."}</p>
            <a href="mailto:leohannolainen999@gmail.com" class="mt-4 inline-block bg-hotmag text-white py-2 px-4 rounded hover:bg-magenta">{"Email me"}</a>
        </section>
    }
}
