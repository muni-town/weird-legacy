use std::fs::File;

use log::{debug, warn};
use serde_json::to_writer;
use tauri::{
    api::{dialog::FileDialogBuilder, path::download_dir},
    command, AppHandle, Manager, State,
};

use crate::{
    prelude::*,
    state::{Link, User},
    utils,
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
pub fn export_zip(handle: AppHandle) -> Result<()> {
    let zip_file = handle
        .path_resolver()
        .app_cache_dir()
        .unwrap()
        .join("website.zip");
    FileDialogBuilder::new()
        .set_file_name("website.zip")
        .set_directory(download_dir().unwrap())
        .save_file(move |f| {
            if let Some(file) = f {
                debug!("Saving website bundle to {}", file.to_str().unwrap());
                std::fs::copy(zip_file, file).unwrap();
            }
        });
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
pub fn get_user(state: State<'_, AppState>, handle: AppHandle) -> Result<User> {
    let user: User = match utils::load_user(handle) {
        Ok(u) => u,
        Err(e) => {
            warn!("Could not load user data: {e}");
            let user: &User = &state.data.lock().unwrap();

            user.clone()
        }
    };
    Ok(user)
}
