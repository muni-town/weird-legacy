use std::fs::File;

use log::debug;
use serde_json::to_writer;
use tauri::{
    api::{dialog::FileDialogBuilder, path::download_dir},
    command, AppHandle, Manager, State,
};

use crate::state::AppState;
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

pub fn zip_dir(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    use std::io::prelude::*;
    use std::io::Write;
    use zip::result::ZipError;
    use zip::write::FileOptions;

    use std::path::Path;
    use walkdir::{DirEntry, WalkDir};

    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    let it: &mut dyn Iterator<Item = DirEntry> = &mut it.filter_map(|e| e.ok());
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(src_dir)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;

    Ok(())
}
