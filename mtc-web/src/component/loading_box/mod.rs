use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

#[component]
pub fn LoadingBoxComponent() -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "flex items-center justify-center grow",
            div { class: "flex flex-row gap-4 w-fit",
                span { class: "loading loading-spinner loading-xl" }
                span { { translate!(i18, "messages.loading") } }
            }
        }
    }
}