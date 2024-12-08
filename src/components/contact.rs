use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="text-center py-16 px-4 bg-custom-bg-special text-custom-main">
            <div class="mx-auto max-w-[366px]">
                <h2 class="text-4xl mb-4">
                    <b>
                        {"Contact me"}
                    </b>
                </h2>
                <p class="text-2xl  font-sans font-extralight">{"I'm always interested in hearing about new studying or working opportunities."}</p>
                <a href="mailto:leohannolainen999@gmail.com" class="mt-8 text-xl  inline-block py-2 px-2 rounded bg-custom-special hover:bg-custom-active">
                {"Email me"}
                </a>
            </div>

        </section>
    }
}
