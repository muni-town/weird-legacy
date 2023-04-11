use std::fs::File;

use log::debug;
use serde_json::to_writer;
use tauri::{
    api::{dialog::FileDialogBuilder, path::download_dir},
    command, AppHandle, Manager, State,
};

use crate::{state::AppState, utils::zip_dir};
use crate::{
    prelude::*,
    state::{Link, User},
};

#[command]
pub fn toggle_preview_window(state: State<'_, AppState>, handle: AppHandle) -> Result<()> {
    if *state.preview.lock().unwrap() {
        if let Some(w) = handle.get_window("preview") {
            w.hide()?;
        }
        *state.preview.lock().unwrap() = false;
    } else {
        if let Some(w) = handle.get_window("preview") {
            w.show()?;
        }

        *state.preview.lock().unwrap() = true;
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

    let user = User {
        name: "user".to_owned(),
        username: "user".to_owned(),
        description: "Weird app user".to_owned(),
        links: (*state.data.lock().unwrap().links).to_vec(),
    };

    to_writer(content_path, &user)?;

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
