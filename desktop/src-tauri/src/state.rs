use std::{path::PathBuf, sync::Mutex};

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
    pub data: Mutex<User>,
}

/// Information needed publish to a backend.
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
#[serde(rename_all = "snake_case")]
pub enum BackendInfo {
    /// Publish a zip to a filesystem path.
    Filesystem { target: PathBuf },
}
