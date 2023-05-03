use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
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
