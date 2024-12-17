use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Project {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub technologies: Vec<(String, String)>,
    pub image_url: String,
    pub links: Vec<(String, String)>,
    pub reverse: bool,
}
