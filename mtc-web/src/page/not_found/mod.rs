use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::record_model::RecordModel;

use crate::APP_STATE;
use crate::router::Route::{DashboardPage, HomePage};

#[component]
pub fn NotFoundPage() -> Element {
    let app_state = APP_STATE.peek();
    let i18 = use_i18();

    let mut breadcrumbs = app_state.breadcrumbs.signal();
    breadcrumbs.set(vec![
        RecordModel { title: translate!(i18, "messages.not_found"), slug: "".to_string() },
    ]);

    rsx! {
        div { class: crate::DIV_CENTER,
            div { class: "flex flex-col self-center m-fit gap-5",
                span { class: "flex justify-center text-9xl text-base-content", "404"}
                span { class: "text-4xl text-base-content", { translate!(i18, "messages.not_found") } }
                div { class: "pt-4 flex justify-center gap-10",
                    Link { class: "btn btn-ghost text-primary", to: HomePage {},
                        Icon {
                            width: 26,
                            height: 26,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::md_action_icons::MdHome
                        }
                        { translate!(i18, "messages.home") }
                    }
                    Link { class: "btn btn-ghost text-accent", to: DashboardPage {},
                        Icon {
                            width: 26,
                            height: 26,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::md_action_icons::MdLogin
                        }
                        { translate!(i18, "messages.sign_in") }
                    }
                }
            }
        }
    }
}
