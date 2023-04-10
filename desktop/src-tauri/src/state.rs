use std::sync::Mutex;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub description: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Default)]
pub struct AppState {
    /// state of the window if it is hidden or shown.
    pub preview: Mutex<bool>,
    pub data: Mutex<User>,
}
