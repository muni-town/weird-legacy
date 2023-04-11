#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ipc::{
    add_link, export_zip, generate_site, remove_link, toggle_preview_window, update_user,
};
use log::debug;
use state::AppState;
use std::{
    fs, io,
    path::{Path, PathBuf},
};
use tauri::{Manager, WindowEvent};

mod ipc;
mod error;
mod prelude;
mod state;

fn main() {
    tauri::Builder::default()
        .on_window_event(|event| {
            if let WindowEvent::CloseRequested { .. } = event.event() {
                if let Some(win) = event.window().get_window("preview") {
                    win.close().unwrap();
                }
            }
        })
        .setup(move |app| {
            let filepath = app
                .path_resolver()
                .resolve_resource("assets/template.zip")
                .expect("error failed to resolve resource dir");
            let target_dir: PathBuf = app
                .path_resolver()
                .app_local_data_dir()
                .unwrap()
                .join("template/");
            if !target_dir.exists() {
                fs::create_dir_all(&target_dir).expect("error creating template directory");
                extract_template(filepath, &target_dir);
            }
            // create the preview window
            tauri::WindowBuilder::new(
                app,
                "preview",
                tauri::WindowUrl::External(
                    format!("file://{}/index.html", target_dir.to_str().unwrap())
                        .parse()
                        .unwrap(),
                ),
            )
            .build()?
            .hide()?;
            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            toggle_preview_window,
            generate_site,
            export_zip,
            remove_link,
            add_link,
            update_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn extract_template(filepath: PathBuf, dest: &Path) {
    let file = fs::File::open(filepath).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => dest.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            debug!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            debug!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}
