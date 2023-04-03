use tauri::{command, State, AppHandle, Manager};

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

