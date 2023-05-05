use std::fs::File;

use serde_json::to_writer;
use tauri::{command, AppHandle, Manager, State};

use crate::{
    backends::{filesystem::FilesystemBackend, PublishingBackend},
    prelude::*,
    state::{BackendInfo, Link, User},
};
use crate::{state::AppState, utils::zip_dir};

#[command]
pub fn toggle_preview_window(handle: AppHandle) -> Result<()> {
    if let Some(w) = handle.get_window("preview") {
        if let Ok(visible) = w.is_visible() {
            if visible {
                w.hide()?;
            } else {
                w.show()?;
            }
        }
    }
    Ok(())
}

#[command]
pub fn generate_site(state: State<'_, AppState>, handle: AppHandle) -> Result<()> {
    let template_dir = handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join("template/");
    let zip_file = handle
        .path_resolver()
        .app_cache_dir()
        .unwrap()
        .join("website.zip");
    let content_path = File::create(template_dir.join("content.json"))?;

    let user: &User = &state.data.lock().unwrap();

    to_writer(content_path, user)?;

    // zip the website bundle.
    zip_dir(
        template_dir.to_str().unwrap(),
        zip_file.to_str().unwrap(),
        zip::CompressionMethod::Deflated,
    )?;

    Ok(())
}

#[command]
pub fn publish_to_backend(backend_info: BackendInfo, handle: AppHandle) -> Result<()> {
    // Export the site to a zip file
    let export_path = handle
        .path_resolver()
        .app_cache_dir()
        .unwrap()
        .join("website.zip");

    match backend_info {
        BackendInfo::Filesystem { target } => {
            FilesystemBackend::publish(&export_path, target)?;
        }
    }

    Ok(())
}

#[command]
pub fn remove_link(url: String, text: String, state: State<'_, AppState>) -> Result<()> {
    state
        .data
        .lock()
        .unwrap()
        .links
        .retain(|v| !(v.url == url && v.text == text));
    Ok(())
}

#[command]
pub fn add_link(url: String, text: String, state: State<'_, AppState>) -> Result<()> {
    state.data.lock().unwrap().links.push(Link { text, url });
    Ok(())
}

#[command]
pub fn update_user(
    name: String,
    username: String,
    description: String,
    state: State<'_, AppState>,
) -> Result<()> {
    let links = (*state.data.lock().unwrap().links).to_vec();
    *state.data.lock().unwrap() = User {
        name,
        username,
        description,
        links,
    };
    Ok(())
}

#[command]
pub fn get_user(state: State<'_, AppState>) -> Result<User> {
    let user: &User = &state.data.lock().unwrap();

    Ok(user.clone())
}
