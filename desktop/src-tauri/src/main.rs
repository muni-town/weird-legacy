#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ipc::{
    add_link, export_zip, generate_site, get_user, remove_link, toggle_preview_window, update_user,
};
use log::debug;
use server::start_server;
use state::AppState;
use std::{
    fs, io,
    path::{Path, PathBuf},
    sync::mpsc::sync_channel,
};
use tauri::{Manager, WindowEvent};

mod error;
mod ipc;
mod prelude;
mod server;
mod state;
mod utils;

fn main() {
    tracing_subscriber::fmt().init();

    let (tx, rx) = sync_channel(1);
    tauri::Builder::default()
        .on_window_event(move |event| match event.window().label() {
            "preview" => {
                if let WindowEvent::CloseRequested { api, .. } = event.event() {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            }
            "main" => {
                if let WindowEvent::CloseRequested { .. } = event.event() {
                    if let Some(win) = event.window().get_window("preview") {
                        win.close().unwrap();
                    }
                    tx.send(1).expect("Failed to send close signal");
                }
            }
            _ => (),
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
            start_server(rx, app);
            // create the preview window
            tauri::WindowBuilder::new(
                app,
                "preview",
                tauri::WindowUrl::External("http://127.0.0.1:7878".parse().unwrap()),
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
            update_user,
            get_user
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
