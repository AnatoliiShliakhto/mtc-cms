use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::component::message_box::{MessageBoxComponent, MessageBoxComponentKind};
use crate::router::Route::{DashboardPage, HomePage};

#[component]
pub fn NotFoundPage() -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "flex flex-col pt-3",
            div { class: "m-fit py-10 self-center",
                MessageBoxComponent { kind: MessageBoxComponentKind::Error, message: translate!(i18, "messages.not_found") }
                div { class: "flex mt-4 gap-4 justify-center",
                    Link { class: "link", to: HomePage {}, { translate!(i18, "messages.home") } }
                    Link { class: "link", to: DashboardPage {}, { translate!(i18, "messages.sign_in") } }
                }
            }
        }
    }
}