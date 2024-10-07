#![allow(non_snake_case, unused_variables)]
use prelude::*;

mod repository;
mod services;
mod error;
mod pages;
mod hooks;
mod elements;
mod icons;
mod utils;
mod components;
mod tasks;
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
        tasks::prelude::*,
        utils::prelude::*,
        pages::prelude::*,
        js_eval::*,
        t,
        page_init,
        url,
        fail,
    };

    pub use mtc_model::prelude::*;
    pub use dioxus::prelude::*;
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
    launch(app);
}

pub fn app() -> Element {
    let auth_state = use_init_auth_state();
    use_init_i18n(I18N_UK_UA);
    use_init_message_box();
    use_init_api_client();
    use_init_breadcrumbs();
    use_init_search_engine();
    use_init_pages_entries();

    /*
        use_init_i18n(|| {
            I18nConfig::new(langid!("en-US"))
                .with_locale(Locale::new_static(
                    langid!("en-US"),
                    include_str!("../../i18n/en-US.ftl"),
                ))
        });
        let mut i18n = i18n();

     */

    use_coroutine(sync_service);
    use_coroutine(message_box_service);

    let sync_eval = eval(EVAL_SYNC);
    let sync_task = use_coroutine_handle::<SyncAction>();

    use_hook(|| {
        sync_task.send(SyncAction::RefreshState("".into()));

        spawn(async move {
            to_owned![sync_eval];
            loop {
                if sync_eval.recv().await.is_ok() {
                    sync_task.send(SyncAction::RefreshState(auth_state().id))
                }
            }
        })
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
    }
}