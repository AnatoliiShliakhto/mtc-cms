use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::component::breadcrumb::Breadcrumb;

#[component]
pub fn Dashboard() -> Element {
    let i18 = use_i18();

    rsx! {
        section { class: "flex grow flex-row",
            div { class: "flex grow flex-col items-center gap-3 p-2 body-scroll",
                div { class: "self-start",
                    Breadcrumb { title: translate!(i18, "messages.administrator") }
                }
            }
        }
    }
}
