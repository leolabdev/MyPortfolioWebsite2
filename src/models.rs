use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Project {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub technologies: Vec<Technology>,
    pub image_url: String,
    pub links: Vec<LinkItem>,
    pub reverse: bool,
}

#[derive(Clone, PartialEq)]
pub struct LinkItem {
    pub url: String,
    pub sr_text: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct Technology {
    pub name: &'static str,
    pub url: &'static str,
}
