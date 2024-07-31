use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::element::migration::Migration;
use crate::router::Route::DashboardPage;

#[component]
pub fn HomePage() -> Element {
    let i18 = use_i18();

    let mut migrate = use_signal(|| false);

    if migrate() {
        return rsx! {
            div { class: crate::DIV_CENTER,
                div { class: "flex w-full flex-col rounded border p-5 shadow-md max-w-96 input-bordered",
                    Migration {}
                }
            }
        };
    }
    
    rsx! {
        div { class: crate::DIV_CENTER,
            p { class: "text-4xl",
                "Military training center content management system!"
            }
            div { class: "flex gap-3 flex-inline",
                button { class: "btn btn-outline btn-success",
                    onclick: move |_| migrate.set(true),
                    { translate!(i18, "messages.migration") }
                }
                Link { class: "w-fit btn btn-neutral btn-outline",
                    to: DashboardPage {},
                    { translate!(i18, "messages.dashboard") }
                }
            }
        }
    }
}
