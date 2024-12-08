use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub image_url: String,
    pub links: Vec<(String, String)>,
}

#[derive(Properties, PartialEq)]
pub struct ProjectsProps {
    pub projects: Vec<Project>,
}

#[function_component(Projects)]
pub fn projects(props: &ProjectsProps) -> Html {
    html! {
        <section id="projects" class="bg-dkblue text-white px-4 py-16 max-w-6xl mx-auto">
            <h2 class="text-2xl mb-8">{"Projects I'm proud of"}</h2>
            <div class="grid gap-8">
                {
                    for props.projects.iter().map(|project| html! {
                        <article class="flex flex-col-reverse lg:flex-row items-center gap-8">
                            <div class="flex-1">
                                <h3 class="text-lg text-aqua">{ &project.title }</h3>
                                <p class="text-sm mb-4">{ &project.description }</p>
                                <h4 class="text-md font-semibold">{"Technologies used:"}</h4>
                                <ul class="list-disc ml-5 mb-4">
                                    { for project.technologies.iter().map(|tech| html! {
                                        <li>{ tech }</li>
                                    }) }
                                </ul>
                                <div class="flex gap-4">
                                    { for project.links.iter().map(|(name, url)| html! {
                                        <a href={url.clone()} target="_blank" class="text-aqua underline hover:text-magenta">{ name }</a>
                                    }) }
                                </div>
                            </div>
                            <img src={project.image_url.clone()} alt={format!("Screenshot of {}", project.title)} class="rounded-lg border-aqua border p-4" />
                        </article>
                    })
                }
            </div>
        </section>
    }
}
