use crate::models::{LinkItem, Project, Technology};

// #[derive(Clone)]
// struct Technology {
//     name: &'static str,
//     url: &'static str,
// }

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            subtitle: "Workki AI".to_string(),
            title: "Web pages".to_string(),
            description: "To date, I have successfully added end-to-end tests with Cypress, significantly enhancing application reliability. I adapted the frontend to meet evolving business needs and migrated static data and blog content to the Sanity CMS, optimizing content management workflows. Additionally, I updated SEO practices to align with the latest standards, resulting in improved search engine visibility and traffic. I continue to contribute to ongoing development and optimization efforts.".to_string(),
            technologies: vec![VUE, NUXT, CYPRESS, SANITY],
            image_url: "assets/img/workki.PNG".to_string(),
            links: vec![
                // ("Website".to_string(), "https://workkiai.com/".to_string()),
                // ("GitHub".to_string(), "https://github.com/example".to_string()),
            ],
            reverse: true,
        },
        Project {
            subtitle: "Thesis Project".to_string(),
            title: "Thesis Project".to_string(),
            description: "Designed and developed a web application for managing device content across LAB UAS.".to_string(),
            technologies: vec![REACT, REDUX, NEST],
            image_url: "assets/img/thesis.PNG".to_string(),
            links: vec![
               LinkItem {
                icon_url: Some("assets/icons/globe.svg".to_string()),
                sr_text: Some("s".to_string()),
                url: "https://www.theseus.fi/handle/10024/812819".to_string()
                }
            ],
            reverse: false,
        },
    ]
}

const VUE: Technology = Technology {
    name: "Vue 3",
    url: "https://vuejs.org/",
};
const NUXT: Technology = Technology {
    name: "Nuxt.js",
    url: "https://nuxtjs.org/",
};
const CYPRESS: Technology = Technology {
    name: "Cypress",
    url: "https://www.cypress.io/",
};
const SANITY: Technology = Technology {
    name: "Sanity CMS",
    url: "https://www.sanity.io/",
};
const REACT: Technology = Technology {
    name: "React",
    url: "https://react.dev/",
};
const REDUX: Technology = Technology {
    name: "Redux Toolkit",
    url: "https://redux-toolkit.js.org/",
};
const NEST: Technology = Technology {
    name: "NestJS",
    url: "https://nestjs.com/",
};
