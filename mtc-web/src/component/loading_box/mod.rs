use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

#[component]
pub fn LoadingBoxComponent() -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "inline-flex items-center gap-3",
            span { class: "loading loading-bars loading-lg" }
            span { { translate!(i18, "messages.loading") } }
        }

            /*
            div { class: "flex w-72 flex-col gap-4",
                div { class: "grid h-32 w-full place-items-center skeleton",
                    div { class: "inline-flex items-center gap-3",
                        span { class: "loading loading-bars loading-lg" }
                        span { { translate!(i18, "messages.loading") } }
                    }
                }
                div { class: "skeleton h-4 w-36" }
                div { class: "skeleton h-4 w-full" }
                div { class: "skeleton h-4 w-full" }

            }
            */
    }
}
