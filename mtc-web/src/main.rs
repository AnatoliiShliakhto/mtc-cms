#![allow(non_snake_case, unused_variables)]
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_std::i18n::{use_i18, use_init_i18n, Language};
use tracing::Level;

use mtc_model::i18n::en_US::EN_US;
use mtc_model::i18n::uk_UA::UK_UA;
use crate::repository::storage::use_persistent;
use crate::router::Route;
use crate::service::health_service::HealthService;
use crate::state::AppState;

//todo refactor to .env
static API_URL: &str = "https://localhost/api";
static PUBLIC_STORAGE_URL: &str = "/files";
static PRIVATE_STORAGE_URL: &str = "/api/private_storage";
static DIV_CENTER: &str = "flex min-h-[70vh] w-full justify-center items-center";
pub static SLUG_PATTERN: &str = "[\\d\\w_\\-]{4,30}"; // [word, digit, -_] {min, max}
pub static TITLE_PATTERN: &str = ".{4,50}"; // any {min, max}

mod component;
mod element;
mod error;
mod handler;
mod model;
mod page;
mod repository;
mod router;
mod service;
mod state;

pub static APP_STATE: GlobalSignal<AppState> = Signal::global(AppState::default);

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App);
}

#[component]
pub fn App() -> Element {
    let app_state = APP_STATE.peek();

    use_init_i18n("en-US".parse().unwrap(), "uk-UA".parse().unwrap(), || {
        let en_us = Language::from_str(EN_US).unwrap();
        let uk_ua = Language::from_str(UK_UA).unwrap();
        vec![en_us, uk_ua]
    });

    let mut i18 = use_i18();

    let user_i18n_en = use_persistent("settings_i18n_en", || true);

    use_effect(move || match user_i18n_en.get().eq(&true) {
        true => i18.set_language("en_US".parse().unwrap()),
        false => i18.set_language("uk_UA".parse().unwrap()),
    });

    let health_eval = eval(
        r#"
        setInterval(() => dioxus.send(), 600000);
        "#,
    );
    
    use_hook(|| {
        let app_state = APP_STATE.peek();
        app_state.service.health_check();
        
        spawn(async move {
            to_owned![health_eval];
            loop {
                if health_eval.recv().await.is_ok() {
                    app_state.service.health_check()
                }
            }
        })
    });


    rsx! {
        Router::<Route> {}
    }
}
