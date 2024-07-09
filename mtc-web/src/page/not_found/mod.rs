use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::component::message_box::{MessageBoxComponent, MessageBoxComponentKind};
use crate::router::Route::{DashboardPage, HomePage};

#[component]
pub fn NotFoundPage() -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "grid w-full place-items-center body-scroll",
            div { class: "self-center py-10 m-fit",
                MessageBoxComponent { kind: MessageBoxComponentKind::Error(translate!(i18, "messages.not_found"))  }
                div { class: "mt-4 flex justify-center gap-4",
                    Link { class: "link", to: HomePage {}, { translate!(i18, "messages.home") } }
                    Link { class: "link", to: DashboardPage {}, { translate!(i18, "messages.sign_in") } }
                }
            }
        }
    }
}