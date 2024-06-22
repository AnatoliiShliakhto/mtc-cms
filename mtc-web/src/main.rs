#![allow(non_snake_case)]

use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_std::i18n::{Language, use_i18, use_init_i18n};
use tracing::Level;

use mtc_model::i18n::en_US::EN_US;
use mtc_model::i18n::uk_UA::UK_UA;

use crate::repository::storage::use_persistent;
use crate::router::Route;
use crate::service::health_service::HealthService;
use crate::state::AppState;

static API_URL: &str = "https://localhost/api";

mod model;
mod component;
mod page;
mod error;
mod state;
mod handler;
mod element;
mod router;
mod repository;
mod service;

pub static APP_STATE: GlobalSignal<AppState> = Signal::global(AppState::default);

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App);
}

pub fn App() -> Element {
    let app_state = APP_STATE.peek();

    use_init_i18n("en-US".parse().unwrap(), "uk-UA".parse().unwrap(), || {
        let en_us = Language::from_str(EN_US).unwrap();
        let uk_ua = Language::from_str(UK_UA).unwrap();
        vec![en_us, uk_ua]
    });

    let mut i18 = use_i18();

    let user_i18n_en =
        use_persistent("settings_i18n_en", || true);

    match user_i18n_en.get().eq(&true) {
        true => i18.set_language("en_US".parse().unwrap()),
        false => i18.set_language("uk_UA".parse().unwrap()),
    }

    app_state.service.health_check();

    rsx! {
        Router::<Route> {}
    }
}
