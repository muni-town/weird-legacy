#![allow(unused)]
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    title: String,
    url: String,
}

impl Link {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PageData {
    url: String,
    github_username: String,
    display_name: String,
    links: Vec<Link>,
}

impl PageData {
    pub fn new(
        url: String,
        github_username: String,
        display_name: String,
        links: Vec<Link>,
    ) -> Self {
        Self {
            url,
            github_username,
            display_name,
            links,
        }
    }
}
