use prelude::*;

mod commands;
mod state;
mod error;

pub mod prelude {
    pub use {
        super::{
            error::*,
            state::*,
            commands::prelude::*
        },
        tauri::{
            State,
            Manager,
            WebviewWindowBuilder,
            WebviewUrl,
            PhysicalSize,
            AppHandle,
            command,
            generate_handler,
            generate_context,
        },
        tauri_plugin_fs::FsExt,
        tauri_plugin_http::reqwest,
        tauri_plugin_shell::ShellExt,
        serde_json::{ Value, json },
        reqwest::{ Client, ClientBuilder, StatusCode, header::HeaderMap },
        std::{ io::Write, path::PathBuf, borrow::Cow },
    };
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            #[cfg(mobile)]
            app.handle()
                .plugin(tauri_plugin_barcode_scanner::init())
                .unwrap();
            #[cfg(mobile)]
            app.handle()
                .plugin(tauri_plugin_view::init())
                .unwrap();
            #[cfg(mobile)]
            app.handle()
                .plugin(tauri_plugin_keep_screen_on::init())
                .unwrap();

            app.manage(AppState::default());
            let webview_url = WebviewUrl::External(env!("FRONT_END_URL").parse().unwrap());
            _ = app.fs_scope()
                .allow_directory(app.path().download_dir().unwrap_or_default(), true);

            let window = WebviewWindowBuilder::new(app, "mtc-app-window", webview_url.clone())
//                .devtools(true)
                .accept_first_mouse(true)
                .enable_clipboard_access()
                .build()?;

            #[cfg(not(mobile))]
            {
                window.set_title(env!("APP_TITLE")).unwrap();
                window
                    .set_min_size(Some(PhysicalSize {
                        width: 640,
                        height: 600,
                    }))
                    .unwrap();
                window.set_resizable(true).unwrap();
                window.set_decorations(true).unwrap();
                window.maximize().unwrap();
            }
            Ok(())
        })
        .invoke_handler(generate_handler![
            get_platform, set_session, download, open_in_browser
        ])
        .run(generate_context!())
        .expect("error while running application");
}