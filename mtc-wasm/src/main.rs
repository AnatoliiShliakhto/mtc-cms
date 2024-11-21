#![allow(non_snake_case, unused_variables)]

use prelude::*;

mod repository;
mod services;
mod error;
mod pages;
mod elements;
mod icons;
mod components;
mod macros;
mod packs;
mod js;
mod application;
mod state;

pub mod prelude {
    pub static FRONT_END_URL: &str = env!("FRONT_END_URL");
    pub static API_ENDPOINT: &str = dioxus_core::const_format::concatcp!(env!("FRONT_END_URL"), "/api");

    pub use crate::{
        state::prelude::*,
        packs::prelude::*, // site packs
        components::prelude::*,
        elements::prelude::*,
        error::*,
        icons::prelude::*,
        repository::prelude::*,
        services::prelude::*,
        pages::prelude::*,
        macros::prelude::*,
        js::*,
        application::prelude::*,
    };

    pub use mtc_common::prelude::*;
    pub use dioxus::prelude::{*, document::{eval, Title}};
    pub use reqwest::{Client, Response};
    pub use chrono::{DateTime, Local};
    pub use futures_util::StreamExt;
    pub use gloo_storage::{LocalStorage, SessionStorage, Storage};
    pub use human_bytes::*;
    pub use magic_crypt::{MagicCryptTrait, new_magic_crypt};

    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{json, Value};
    pub use std::{
        borrow::Cow,
        collections::{BTreeMap, BTreeSet},
        str::FromStr,
        iter::zip,
        ffi::OsStr,
        path::Path,
    };
    pub use wasm_bindgen::JsCast;
    pub use tracing::error;
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(|| {
        let session = use_init_state(I18N_UK_UA);

        // register ServiceWorker with SessionId
        eval(&JS_SW_REGISTER.replace("{session}", &session()));

        use_hook(move || spawn(async move {
            if !is_tauri() { return }
            let Ok(result) = tauri_invoke_without_args("get_platform")
                .await else { return };
            let Some(platform) = result.as_string() else { return };
            set_tauri_session(session().to_string()).await;
            state!(set_platform, platform.into());
        }));

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
                if let Ok(Value::Bool(sw_present)) = eval(JS_SW_CHECK).recv().await {
                    if sw_present {
                        sync_task.send(SyncAction::RefreshState())
                    } else {
                        error_dialog!("error-sw-unsupported")
                    }
                }
            });
        });

        rsx! {
            Title { { t!("site-title") } }
            ErrorBoundary {
                handle_error: |errors: ErrorContext| {
                    match errors.show() {
                        Some(view) => view,
                        None => rsx! {
                            pre {
                                color: "red",
                                "Oops, we ran into an error\n{errors:#?}"
                            }
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
                    onmounted: |_| { eval(JS_SCROLL_UP); },
                    "onclick": "window.scrollTo(0, 0)",
                    Icon { icon: Icons::ArrowUp, class: "size-8" }
                }
            }
            DialogBox {}
        }
    });
}