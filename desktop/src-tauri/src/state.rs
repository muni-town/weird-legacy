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

pub type Links = Vec<Link>;

#[derive(Debug, Default, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub title: String,
    pub about: String,
    pub photo: Option<String>,
}

/// Contains data used to generate the final `index.html` file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub profile: Profile,
    pub links: Links,
}

/// contains the app global state
#[derive(Debug, Default)]
pub struct AppState {
    pub profile: Mutex<Profile>,
    pub links: Mutex<Links>,
}
