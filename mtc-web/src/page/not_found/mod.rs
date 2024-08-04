use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::record_model::RecordModel;

use crate::APP_STATE;

#[component]
pub fn NotFoundPage() -> Element {
    let app_state = APP_STATE.peek();
    let i18 = use_i18();

    let mut breadcrumbs = app_state.breadcrumbs.signal();

    use_effect(move || {
        breadcrumbs.set(vec![
            RecordModel { title: translate!(i18, "messages.not_found"), slug: "".to_string() },
        ]);
    });

    rsx! {
        div { class: crate::DIV_CENTER,
            div { class: "flex flex-col self-center m-fit gap-5",
                span { class: "flex justify-center text-9xl text-base-content", "404"}
                span { class: "text-4xl text-base-content", { translate!(i18, "messages.not_found") } }
            }
        }
    }
}
