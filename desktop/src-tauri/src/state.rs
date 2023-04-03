use std::sync::Mutex;

#[derive(Debug, Default)]
pub struct PreviewWindow {
    /// state of the window if it is hidden or shown.
    pub state: Mutex<bool>,
}

