use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

#[component]
pub fn LoadingBoxComponent() -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "grid w-full place-items-center body-scroll",
            div { class: "flex w-fit flex-row gap-4",
                span { class: "loading loading-spinner loading-xl" }
                span { { translate!(i18, "messages.loading") } }
            }
        }
    }
}