use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="bg-custom-bg-add text-custom-main text-lg font-mono py-16 px-4">
            <ul class="flex flex-wrap justify-center items-center gap-8 max-w-[1200px] mx-auto">
                <li class="flex-1 ">
                    <h1 class="font-sans text-2xl group hover:text-custom-active inline-flex">
                        <a href="/" class="">
                         <span class="fab fa-node-js pr-2 text-custom-add group-hover:text-custom-active" aria-hidden="true"></span>
                             {"Leo Hannolainen"}
                        </a>
                    </h1>
                </li>
                <li><a href="#projects" class="hover:text-custom-active">{"Projects"}</a></li>
                <li><a href="#contact" class="hover:text-custom-active">{"Contact"}</a></li>
                <li class="color-custom-add">
                    <a href="https://linkedin.com/in/leo-hannolainen-860859205" target="_blank" class="rounded">
                        <span class="fab fa-linkedin text-custom-add hover:text-custom-active text-[150%]"  aria-hidden="true"></span>
                        <span class="sr-only">{"LinkedIn"}</span>
                    </a>
                </li>
                <li>
                    <a href="https://github.com/leolab1337" target="_blank" class="rounded">
                        <span class="fab fa-github-square text-custom-add hover:text-custom-active text-[150%]" aria-hidden="true"></span>
                        <span class="sr-only">{"GitHub"}</span>
                    </a>
                </li>
                <li>
                    <a href="https://drive.google.com/file/d/1roSodsRcVNLScHU9ZzziS8zeNP9hSjjl" target="_blank" class="bg-hotmag py-2 px-4 rounded bg-custom-special hover:bg-custom-active">
                        {"Resume"}
                    </a>
                </li>
            </ul>
        </nav>
    }
}
