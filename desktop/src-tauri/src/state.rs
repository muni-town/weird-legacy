use std::{sync::Mutex, path::PathBuf};

use serde::{Deserialize, Serialize};
use toml::{Table, Value, value::Array};
use ts_rs::TS;

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Link {
    pub id: usize,
    pub text: String,
    pub url: String,
}

impl Link {
    pub fn to_value(&self) -> Value {
        let mut table = Table::new();

        table.insert("id".into(), Value::Integer(self.id as i64));
        table.insert("text".into(), Value::String(self.text.clone()));
        table.insert("url".into(), Value::String(self.url.clone()));

        Value::Table(table)
    }
}

pub type Links = Vec<Link>;

impl From<&Links> for W<Value> {
    fn from(value: &Links) -> Self {
        let mut links = Array::new();

        for val in value {
            links.push(val.to_value());
        }

        W(Value::Array(links))
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../src/bindings/")]
pub struct Profile {
    pub name: String,
    pub username: String,
    pub title: String,
    pub about: String,
    pub photo: Option<String>,
}

impl Profile {
    pub fn to_value(&self) -> toml::Value {
        let mut table = Table::new();

        table.insert("name".into(), Value::String(self.name.clone()));
        table.insert("username".into(), Value::String(self.username.clone()));
        table.insert("title".into(), Value::String(self.title.clone()));
        table.insert("about".into(), Value::String(self.about.clone()));
        table.insert("photo".into(), Value::String(self.photo.clone().unwrap_or_default()));

        Value::Table(table)
    }
}

/// Contains data used to generate the final `index.html` file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub profile: Profile,
    pub links: Links,
}

/// contains app configurations
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub template_path: Option<PathBuf>,
}

/// contains the app global state
#[derive(Debug, Default)]
pub struct AppState {
    pub profile: Mutex<Profile>,
    pub links: Mutex<Links>,
    pub config: Mutex<Config>,
}
