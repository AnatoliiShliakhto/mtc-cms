use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;

use crate::repository::storage::use_persistent;

//todo: multilanguage support
#[component]
pub fn LanguageSwitcherComponent() -> Element {
    let mut i18 = use_i18();

    let mut user_i18n_en =
        use_persistent("settings_i18n_en", || true);

    rsx! {
        div { class: "btn btn-ghost join-item",
            prevent_default: "onclick",
            onclick: move |_| {
                user_i18n_en.set(!user_i18n_en.get());
                if user_i18n_en.get() {
                    i18.set_language("en-US".parse().unwrap())
                } else {
                    i18.set_language("uk-UA".parse().unwrap())
                }
            },
            label { class: "swap",
                input {
                    r#type: "checkbox",
                    checked: !user_i18n_en.get(),
                }
                div { class: "swap-on", "EN" }
                div { class: "swap-off", "UA" }
            }
        }
    }
}