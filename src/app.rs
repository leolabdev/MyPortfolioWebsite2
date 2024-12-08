use crate::components::contact::Contact;
use crate::components::footer::Footer;
use crate::components::gradient::Gradient;
use crate::components::intro::Intro;
use crate::components::nav::Nav;
use crate::components::projects::{Project, Projects};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let projects = vec![
        Project {
            title: "Workki AI".to_string(),
            description:
                "Added end-to-end tests with Cypress and migrated static data to Sanity CMS."
                    .to_string(),
            technologies: vec![
                "Vue 3".to_string(),
                "Nuxt.js".to_string(),
                "Cypress".to_string(),
                "Sanity CMS".to_string(),
            ],
            image_url: "assets/img/workki.PNG".to_string(),
            links: vec![
                ("Website".to_string(), "https://workkiai.com/".to_string()),
                (
                    "GitHub".to_string(),
                    "https://github.com/example".to_string(),
                ),
            ],
            reverse: false,
            subtitle: "Workki AI".to_string(),
        },
        Project {
            title: "Thesis Project".to_string(),
            description: "Designed and developed a web application for managing device content across LAB UAS.".to_string(),
            technologies: vec!["React".to_string(), "Redux Toolkit".to_string(), "NestJS".to_string()],
            image_url: "assets/img/thesis.PNG".to_string(),
            links: vec![
                ("Thesis Report".to_string(), "https://www.theseus.fi/handle/10024/812819".to_string()),
            ],
            reverse: true,
            subtitle: "test".to_string()
        },
    ];

    html! {
        <div class="bg-custom-bg-main">
            <Nav />
            <Intro/>
            <Gradient />
            <Projects projects={projects} />
            <Gradient />
            <Contact/>
            <Gradient />
            <Footer/>
        </div>
    }
}
