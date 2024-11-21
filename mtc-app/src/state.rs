use crate::prelude::*;

#[derive(Default)]
pub struct AppState {
    pub client: tauri::async_runtime::Mutex<Client>,
}