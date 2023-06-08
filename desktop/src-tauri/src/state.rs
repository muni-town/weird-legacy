use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Link {
    pub id: usize,
    pub text: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct User {
    pub name: String,
    pub username: String,
    pub title: String,
    pub about: String,
    pub photo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub user: User,
    pub links: Vec<Link>,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub user: Mutex<User>,
    pub links: Mutex<Vec<Link>>,
}
