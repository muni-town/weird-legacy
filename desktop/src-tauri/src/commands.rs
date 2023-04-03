use std::fs::File;

use serde::{Deserialize, Serialize};
use serde_json::to_writer;
use tauri::{
    api::{dialog::FileDialogBuilder, path::download_dir},
    command, AppHandle, Manager, State,
};

use crate::state::PreviewWindow;

#[command]
pub fn toggle_preview_window(preview: State<'_, PreviewWindow>, handle: AppHandle) -> bool {
    let mut res = false;
    if *preview.state.lock().unwrap() {
        if let Some(w) = handle.get_window("preview") {
            res = w.hide().is_ok();
        }
        *preview.state.lock().unwrap() = false;
    } else {
        if let Some(w) = handle.get_window("preview") {
            res = w.show().is_ok();
        }

        *preview.state.lock().unwrap() = true;
    }
    res
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    pub text: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub description: String,
    pub links: Vec<Link>,
}

#[command]
pub fn generate_site(links: Vec<Link>, handle: AppHandle) -> Result<(), String> {
    let template_dir = handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join("template/");
    let content_path = File::create(template_dir.join("content.json"))
        .map_err(|e| format!("Error creating content.json: {e}"))?;

    let user = User {
        name: "user".to_owned(),
        description: "Weird app user".to_owned(),
        links,
    };

    to_writer(content_path, &user).map_err(|e| format!("Error writing to content.json: {e}"))?;

    FileDialogBuilder::new()
        .set_file_name("website.zip")
        .set_directory(download_dir().unwrap())
        .save_file(move |f| {
            if let Some(file) = f {
                println!("Saving website bundle to {}", file.to_str().unwrap());
                // zip the website bundle.
                if let Err(e) = zip_dir(
                    template_dir.to_str().unwrap(),
                    file.to_str().unwrap(),
                    zip::CompressionMethod::Deflated,
                ) {
                    println!("Error saving zip file to {}: {e}", file.to_str().unwrap());
                }
            }
        });

    Ok(())
}

fn zip_dir(
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
