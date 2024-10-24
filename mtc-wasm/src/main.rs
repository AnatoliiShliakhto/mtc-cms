#![allow(non_snake_case, unused_variables)]

use prelude::*;

mod repository;
mod services;
mod error;
mod pages;
mod hooks;
mod elements;
mod icons;
mod components;
mod macros;
mod packs;
mod js_eval;

pub mod prelude {
    pub use crate::{
        packs::prelude::*, // various site packs
        components::prelude::*,
        elements::prelude::*,
        error::*,
        hooks::prelude::*,
        icons::prelude::*,
        repository::prelude::*,
        services::prelude::*,
        pages::prelude::*,
        macros::prelude::*,
        js_eval::*,
    };

    pub use mtc_model::prelude::*;
    pub use dioxus::prelude::{*, document::{eval, Title}};
    pub use reqwest::{Client, Response};
    pub use chrono::{DateTime, Local};
    pub use futures_util::StreamExt;
    pub use gloo_storage::{LocalStorage, SessionStorage, Storage};
    pub use human_bytes::*;
    //pub use dioxus_i18n::{prelude::*, t, unic_langid::langid};

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
    pub use tracing::error;
}

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    launch(|| {
        let auth_state = use_init_auth_state();
        use_init_app_state();
        use_init_i18n(I18N_EN_US);
        use_init_dialog_box();
        use_init_api_client();
        use_init_breadcrumbs();
        use_init_search_engine();
        use_init_pages_entries();
        use_init_personnel();
        use_init_personnel_columns();

        use_coroutine(sync_service);

        let sync_task = use_coroutine_handle::<SyncAction>();
        let mut sync_eval = eval(EVAL_SYNC);

        use_hook(|| {
            sync_task.send(SyncAction::RefreshState("".into()));

            spawn(async move {
                loop {
                    if sync_eval.recv::<i32>().await.is_ok() {
                        sync_task.send(SyncAction::RefreshState(auth_state().id))
                    }
                }
            })
        });

        use_effect(|| { eval(EVAL_SCROLL_UP); });

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
            button {
                id: "scrollUpButton",
                class: "fixed btn btn-circle btn-neutral opacity-60 hover:opacity-100",
                class: "right-4 bottom-4 hidden",
                "onclick": "window.scrollTo(0, 0)",
                Icon { icon: Icons::ArrowUp, class: "size-8" }
            }
            DialogBox {}
        }
    });
}