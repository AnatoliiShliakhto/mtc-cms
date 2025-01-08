use crate::prelude::*;

/// Application state
#[derive(Default)]
pub struct AppState {
    pub client: tauri::async_runtime::Mutex<Client>,
}