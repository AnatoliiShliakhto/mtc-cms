use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::record_model::RecordModel;

use crate::APP_STATE;
use crate::page::not_found::NotFoundPage;

#[component]
pub fn AdministratorDashboardPage() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    if !auth_state.is_permission("administrator") {
        return rsx! { NotFoundPage {} };
    }

    let mut breadcrumbs = app_state.breadcrumbs.signal();
    breadcrumbs.set(
        vec![
            RecordModel { title: translate!(i18, "messages.administrator"), slug: "/administrator".to_string() },
        ]
    );

    rsx! {
        section { class: "w-full flex-grow p-3",
            div { class: crate::DIV_CENTER,
                span { "In-Dev" }
            }
        }
    }
}
