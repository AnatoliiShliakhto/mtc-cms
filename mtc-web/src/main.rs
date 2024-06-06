#![allow(non_snake_case)]

use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_std::i18n::{Language, use_i18, use_init_i18n};
use tracing::Level;

use crate::action::health_action::HealthAction;
use crate::i18n::en_US::EN_US;
use crate::i18n::uk_UA::UK_UA;
use crate::router::Route;
use crate::service::auth_service::auth_service;
use crate::service::health_service::health_service;

mod model;
mod widget;
mod page;
mod error;
mod state;
mod action;
mod service;
mod handler;
mod global_signal;
mod component;
mod router;
mod i18n;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App);
}

fn App() -> Element {
    use_init_i18n("en-US".parse().unwrap(), "uk-UA".parse().unwrap(), || {
        let en_us = Language::from_str(EN_US).unwrap();
        let uk_ua = Language::from_str(UK_UA).unwrap();
        vec![en_us, uk_ua]
    });
    let mut i18 = use_i18();

    //todo: language_switcher routine
    i18.set_language("uk-UA".parse().unwrap());

    use_coroutine(health_service);
    use_coroutine(auth_service);

    use_coroutine_handle::<HealthAction>().send(HealthAction::Check);

    rsx! {
        Router::<Route> {}
    }
}
