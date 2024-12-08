use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Project {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub image_url: String,
    pub links: Vec<(String, String)>,
    pub reverse: bool,
}

#[derive(Properties, PartialEq)]
pub struct ProjectsProps {
    pub projects: Vec<Project>,
}

#[function_component(Projects)]
pub fn projects(props: &ProjectsProps) -> Html {
    html! {
        <section id="projects" class="text-custom-main px-4 py-16 max-w-6xl mx-auto">
            <h2 class="text-2xl mb-8">{"Projects I'm proud of"}</h2>
            <div class="grid gap-8">
                { for props.projects.iter().map(|project| {
                    let reverse_class = if project.reverse { "lg:flex-row-reverse" } else { "lg:flex-row" };
                    let border_class = if project.reverse {
                         "border-r border-t border-r-custom-add border-t-custom-add p-4"
                    } else {
                         "border-l border-t border-l-custom-add border-t-custom-add p-4"
                    };
                    html! {
                        // <article class={format!("flex flex-col {} items-center gap-8", reverse_class)}>

                        <article class="grid gap-4 sm:grid-cols-10">
                           <div class="col-span-6 col-start-5 row-start-1 row-end-2 order-2 text-right">
                                <h3 class="text-sm">{ &project.subtitle }</h3>
                                <h4 class="text-lg text-custom-add">{ &project.title }</h4>
                                <p class="text-base mb-4 p-4 bg-custom-bg-add rounded-lg ">{ &project.description }</p>
                            </div>

                            <img
                                src={project.image_url.clone()}
                                alt={format!("Screenshot of {}", project.title)}
                                class={format!("rounded-3xl col-span-5 col-start-1 row-start-1 row-end-2 {}", border_class)}
                            />


                        </article>
                    }
                })}
            </div>
        </section>
    }
}

// <h4 class="text-md font-semibold">{"Technologies used:"}</h4>
//                                 <ul class="list-disc ml-5 mb-4">
//                                     { for project.technologies.iter().map(|tech| html! {
//                                         <li>{ tech }</li>
//                                     }) }
//                                 </ul>
//                                 <div class="flex gap-4">
//                                     { for project.links.iter().map(|(name, url)| html! {
//                                         <a href={url.clone()} target="_blank" class="hover:text-custom-active">{ name }</a>
//                                     }) }
//                                 </div>
