use yew::prelude::*;

#[function_component(Intro)]
pub fn intro() -> Html {
    html! {
        <header id="intro" class="px-4 pt-16 pb-40 mx-auto text-left bg-custom-bg-add text-custom-main">
            <div class="max-w-[1200px] mx-auto">
                <p class="font-mono text-lg">
                    {"Hi, my name is"}
                    <br/>
                    <span class="font-sans text-6xl text-custom-add">
                        {"Leo Hannolainen"}
                    </span>
                </p>
                <h2 class="text-5xl font-light">
                    {"I'm a web developer with a passion for building scalable web applications"}
                </h2>
                <p class="mt-4 text-lg leading-6">
                    {
                        "With a Bachelor's Degree in Software Engineering, I have developed a strong foundation in web development, focusing on both front-end and back-end technologies.
                        I possess expertise in HTML, CSS (SCSS & CSS Modules), JavaScript (TypeScript), and hands-on experience with frameworks like React (Next.js), Angular (Universal), and Vue (Nuxt).
                        Additionally, I am skilled in building server-side solutions using Express, NestJS, and Swagger, with database management experience in SQL (TypeORM) and MongoDB/Mongoose."
                    }
                </p>

                <p class="mt-4 text-lg leading-6">
                    {
                        "I am always eager to learn and embrace new challenges, striving to create innovative and user-friendly digital solutions."
                    }
                </p>



            </div>
        </header>
    }
}
