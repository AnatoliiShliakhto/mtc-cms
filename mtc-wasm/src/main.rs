#![allow(non_snake_case, unused_variables)]

use prelude::*;
use wasm_bindgen_futures::JsFuture;

mod application;
mod components;
mod elements;
mod error;
mod icons;
mod js;
mod macros;
mod packs;
mod pages;
mod repository;
mod services;
mod state;

pub mod prelude {
    pub static FRONT_END_URL: LazyLock<String> = LazyLock::new(|| {
        if cfg!(debug_assertions) {
            "http://localhost:8080".to_string()
        } else {
            web_sys::window()
                .and_then(|w| w.location().href().ok())
                .unwrap_or_default().trim_end_matches('/').to_string()
        }
    });
    pub static API_ENDPOINT: LazyLock<String> = LazyLock::new(|| {
        let url = if cfg!(debug_assertions) {
            "http://localhost:8080".to_string()
        } else {
            web_sys::window()
                .and_then(|w| w.location().href().ok())
                .unwrap_or_default().trim_end_matches('/').to_string()
        };
        format!("{url}/api")
    });

    pub use crate::{
        application::prelude::*,
        components::prelude::*,
        elements::prelude::*,
        error::*,
        icons::prelude::*,
        js::*,
        macros::prelude::*,
        packs::prelude::*, // site packs
        pages::prelude::*,
        repository::prelude::*,
        services::prelude::*,
        state::prelude::*,
    };

    pub use chrono::{DateTime, Local};
    pub use dioxus::prelude::*;
    pub use futures_util::StreamExt;
    pub use gloo_storage::{LocalStorage, SessionStorage, Storage};
    pub use human_bytes::*;
    pub use magic_crypt::{new_magic_crypt, MagicCryptTrait};
    pub use mtc_common::prelude::*;
    pub use reqwest::{Client, Response};

    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{json, Value};
    use std::sync::LazyLock;
    pub use std::{
        borrow::Cow,
        collections::{BTreeMap, BTreeSet},
        ffi::OsStr,
        iter::zip,
        path::Path,
        str::FromStr,
    };
    pub use tracing::error;
    pub use wasm_bindgen::JsCast;
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(|| {
        let session = use_init_state(I18N_UK_UA);

        // register ServiceWorker with SessionId
        spawn(async move {
            if JsFuture::from(jsFfiInitializeServiceWorker(&*session())).await.is_err() {
                error!("failed to invoke jsFfiInitializeServiceWorker");
            }
        });

        use_hook(move || {
            spawn(async move {
                if !is_tauri() {
                    return;
                }
                let Ok(result) = tauri_invoke_without_args("get_platform").await else {
                    return;
                };
                let Some(platform) = result.as_string() else {
                    return;
                };
                set_tauri_session(session().to_string()).await;
                state!(set_platform, platform.into());
            })
        });

        use_hook(move || {
            spawn(async move {
                let database = indexed_db().await.ok();
                state!(set_indexed_db, database);
            })
        });

        use_effect(move || {
            let search_engine = state_fn!(search_engine);
            let pattern = search_engine.pattern;
            let index = search_engine.index;
            let list = search_engine.list;
            let mut result = search_engine.result;

            if pattern().is_empty() {
                *result.write() = vec![];
                return;
            }

            let mut search_res = vec![];
            for idx in index().search(&pattern()).iter().take(30) {
                if let Some(item) = list().get(&idx) {
                    search_res.push(item.to_owned());
                }
            }
            *result.write() = search_res;
        });

        let sync_task = use_coroutine(sync_service);

        use_hook(move || {
            spawn(async move {
                match JsFuture::from(jsFfiCheckServiceWorker()).await {
                    Ok(_) => sync_task.send(SyncAction::RefreshState()),
                    Err(_) => error_dialog!("error-sw-unsupported"),
                }
            });
        });
        jsFfiSetTitle(t!("site-title").as_ref());
        rsx! {
            document::Stylesheet {
                href: asset!("/assets/css/tailwind.css")
            }
            ErrorBoundary {
                handle_error: |errors: ErrorContext| {
                    rsx! {
                        pre {
                            color: "red",
                            "Oops, we ran into an error\n{errors:#?}"
                        }
                    }
                },
                Router::<Route> {}
            }
            div {
                class: "fixed right-4 bottom-4 qr-element",
                button {
                    id: "scrollUpButton",
                    class: "btn btn-circle btn-neutral opacity-60 hover:opacity-100 hidden",
                    onmounted: |_| { jsFfiInitializeScrollUpButton() },
                    Icon { icon: Icons::ArrowUp, class: "size-8" }
                }
            }
            DialogBox {}
        }
    });
}
