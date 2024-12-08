use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="bg-custom-bg-add text-white font-mono text-sm py-16 px-4">
            <ul class="flex flex-wrap justify-center items-center gap-8">
                <li class="flex-1 ">
                    <h1 class="font-sans text-lg group hover:text-custom-active inline-flex">
                        <a href="/" class="">
                            <span class="fab fa-node-js pr-1 text-custom-add group-hover:text-custom-active" aria-hidden="true"></span>
                             {"Leo Hannolainen"}
                        </a>
                    </h1>
                </li>
                <li><a href="#projects" class="hover:text-hotmag">{"Projects"}</a></li>
                <li><a href="#contact" class="hover:text-hotmag">{"Contact"}</a></li>
                <li class="color-custom-add">
                    <a href="https://linkedin.com/in/leo-hannolainen-860859205" target="_blank" class="hover:text-hotmag">
                        <span class="fab fa-linkedin text-custom-add" aria-hidden="true"></span>
                        <span class="sr-only">{"LinkedIn"}</span>
                    </a>
                </li>
                <li>
                    <a href="https://github.com/leolab1337" target="_blank" class="hover:text-hotmag">
                        <span class="fab fa-github-square text-custom-add" aria-hidden="true"></span>
                        <span class="sr-only">{"GitHub"}</span>
                    </a>
                </li>
                <li>
                    <a href="https://drive.google.com/file/d/1roSodsRcVNLScHU9ZzziS8zeNP9hSjjl" target="_blank" class="bg-hotmag text-white py-2 px-4 rounded hover:bg-magenta">
                        {"Resume"}
                    </a>
                </li>
            </ul>
        </nav>
    }
}
