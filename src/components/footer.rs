use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="text-center py-16 text-custom-main bg-custom-bg-add">
            <h2 class="text-3xl font-semibold">{"Leo Hannolainen · Web developer"}</h2>
            <ul class="flex justify-center gap-12 mt-8 text-2xl">
                <li>
                    <a href="https://linkedin.com/in/leo-hannolainen-860859205" target="_blank" rel="noopener noreferrer">
                        <span class="fab fa-linkedin hover:text-custom-active text-[200%]"></span>
                    </a>
                </li>
                <li>
                    <a href="https://github.com/leolabdev" target="_blank" rel="noopener noreferrer">
                        <span class="fab fa-github-square hover:text-custom-active text-[200%]"></span>
                    </a>
                </li>
                <li>
                    <a href="mailto:leohannolainen999@gmail.com">
                        <span class="fas fa-envelope hover:text-custom-active text-[200%]"></span>
                    </a>
                </li>
            </ul>
            <p class="mt-8 text-base">{"© 2024 Leo Hannolainen. All rights reserved."}</p>
        </footer>
    }
}
