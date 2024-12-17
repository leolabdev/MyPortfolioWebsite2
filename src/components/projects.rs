use yew::prelude::*;
use crate::models::Project;

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
                    let (text_col_start, image_col_start, text_align_class, border_class) = if project.reverse {
                        (
                            "col-start-1",
                            "col-start-6",
                            "sm:text-left",
                            "border-l sm:border-t border-t sm:border-b-0 border-l-custom-add border-t-custom-add sm:border-t-custom-add p-4"
                        )
                    } else {
                        (
                            "col-start-5",
                            "col-start-1",
                            "text-right",
                            "border-r border-t sm:border-b-0 sm:border-t border-r-custom-add border-t-custom-add sm:border-t-custom-add p-4"
                        )
                    };

                    html! {
                        <article class="sm:grid gap-4 sm:grid-cols-10 relative">
                        <div class={format!("col-span-6 {} row-start-1 row-end-2 order-2 {} relative z-10", text_col_start, text_align_class)}>
                            <h3 class="text-sm">{ &project.subtitle }</h3>
                            <h4 class="text-lg text-custom-add">{ &project.title }</h4>
                            <p class="text-base mb-4 p-4 bg-custom-bg-add rounded-lg">{ &project.description }</p>
                            <h4 class="font-semibold">{"Technologies used include:"}</h4>
                            <ul class={format!("flex gap-3 text-base{}", if project.reverse { "justify-start" } else { "justify-end justify-end" })}>
                               { for project.technologies.iter().map(|tech| {
                                html! {
                                    <li class="hover:text-custom-active">
                                        <a href={tech.url} target="_blank" rel="noopener noreferrer">
                                            { tech.name }
                                         </a>
                                    </li>
                                }
                                })}
                            </ul>
                        </div>
                            <img
                                src={project.image_url.clone()}
                                alt={format!("Screenshot of {}", project.title)}
                                class={format!("rounded-3xl col-span-5 {} row-start-1 row-end-2 {}", image_col_start, border_class)}
                            />
                        </article>
                    }
                })}
            </div>
        </section>
    }
}
