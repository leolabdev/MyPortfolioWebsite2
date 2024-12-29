use crate::components::contact::Contact;
use crate::components::footer::Footer;
use crate::components::gradient::Gradient;
use crate::components::intro::Intro;
use crate::components::nav::Nav;
use crate::components::projects::Projects;
use crate::services::project_service;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let projects = project_service::get_projects();
    html! {
        <div class="bg-custom-bg-main">
            <Nav />
            <Intro />
            <Gradient />
            <Projects projects={projects} />
            <Gradient />
            <Contact />
            <Gradient />
            <Footer />
        </div>
    }
}
