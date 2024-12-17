use crate::models::Project;

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            title: "Web pages".to_string(),
            description: "To date, I have successfully added end-to-end tests with Cypress, significantly enhancing application reliability. I adapted the frontend to meet evolving business needs and migrated static data and blog content to the Sanity CMS, optimizing content management workflows. Additionally, I updated SEO practices to align with the latest standards, resulting in improved search engine visibility and traffic. I continue to contribute to ongoing development and optimization efforts.".to_string(),
            technologies: vec![
                ("Vue 3".to_string(), "https://vuejs.org/".to_string()),
                ("Nuxt.js".to_string(), "https://nuxtjs.org/".to_string()),
                ("Cypress".to_string(), "https://www.cypress.io/".to_string()),
                ("Sanity CMS".to_string(), "https://www.sanity.io/".to_string()),
            ],
            image_url: "assets/img/workki.PNG".to_string(),
            links: vec![
                ("Website".to_string(), "https://workkiai.com/".to_string()),
                ("GitHub".to_string(), "https://github.com/example".to_string()),
            ],
            reverse: true,
            subtitle: "Workki AI".to_string(),
        },
        Project {
            title: "Thesis Project".to_string(),
            description: "Designed and developed a web application for managing device content across LAB UAS.".to_string(),
            technologies: vec![
                ("React".to_string(), "https://react.dev/".to_string()),
                ("Redux Toolkit".to_string(), "https://redux-toolkit.js.org/".to_string()),
                ("NestJS".to_string(), "https://nestjs.com/".to_string()),
            ],
            image_url: "assets/img/thesis.PNG".to_string(),
            links: vec![
                ("Thesis Report".to_string(), "https://www.theseus.fi/handle/10024/812819".to_string()),
            ],
            reverse: false,
            subtitle: "Thesis Project".to_string(),
        },
    ]
}
