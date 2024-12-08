use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="text-center py-8 text-white">
            <h2 class="text-xl">{"Leo Hannolainen · Web developer"}</h2>
            <ul class="flex justify-center gap-4 mt-4 text-2xl">
                <li>
                    <a href="https://linkedin.com/in/leo-hannolainen-860859205" target="_blank" rel="noopener noreferrer">
                        <span class="fab fa-linkedin"></span>
                    </a>
                </li>
                <li>
                    <a href="https://github.com/leolabdev" target="_blank" rel="noopener noreferrer">
                        <span class="fab fa-github-square"></span>
                    </a>
                </li>
                <li>
                    <a href="mailto:leohannolainen999@gmail.com">
                        <span class="fas fa-envelope"></span>
                    </a>
                </li>
            </ul>
            // <p class="mt-4 text-sm">&copy; 2024 Leo Hannolainen. All rights reserved.</p>
        </footer>
    }
}