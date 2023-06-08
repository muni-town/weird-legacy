#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ipc::{
    add_link, export_zip, generate_site, get_export_zip_base64, get_links, get_user, remove_link,
    toggle_preview_window, update_user,
};
use server::start_server;
use state::AppState;
use std::{
    fs,
    path::PathBuf,
    sync::{mpsc::sync_channel, Mutex},
};
use tauri::{Manager, WindowEvent};

mod error;
mod ipc;
mod prelude;
mod server;
mod state;
mod utils;
mod generator;

fn main() {
    tracing_subscriber::fmt().init();
    let exit = Mutex::new(false);

    let (tx, rx) = sync_channel(1);
    tauri::Builder::default()
        .on_window_event(move |event| match event.window().label() {
            "preview" => {
                if let WindowEvent::CloseRequested { api, .. } = event.event() {
                    event.window().hide().unwrap();
                    if !*exit.lock().unwrap() {
                        api.prevent_close();
                    }
                }
            }
            "main" => {
                if let WindowEvent::CloseRequested { .. } = event.event() {
                    if let Some(win) = event.window().get_window("preview") {
                        win.close().unwrap();
                    }
                    *exit.lock().unwrap() = true;
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
                utils::extract_template(filepath, &target_dir);
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
            let config_path = app.path_resolver().app_config_dir().unwrap();
            if !config_path.exists() {
                fs::create_dir_all(&config_path).expect("error creating config directory");
            }
            Ok(())
        })
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            toggle_preview_window,
            generate_site,
            get_export_zip_base64,
            export_zip,
            remove_link,
            add_link,
            update_user,
            get_user,
            get_links
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
