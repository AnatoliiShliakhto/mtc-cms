use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::record_model::RecordModel;

use crate::APP_STATE;
use crate::element::user_dashboard::UserDashboard;

#[component]
pub fn DashboardPage() -> Element {
    let app_state = APP_STATE.peek();
    let i18 = use_i18();

    let mut breadcrumbs = app_state.breadcrumbs.signal();
    breadcrumbs.set(
        if APP_STATE.peek().auth.signal().read().is_auth() {
            vec![
                RecordModel { title: translate!(i18, "messages.settings"), slug: "/dashboard".to_string() }
            ]
        } else {
            vec![
                RecordModel { title: translate!(i18, "messages.sign_in"), slug: "/dashboard".to_string() }
            ]
        }
    );

    rsx! {
        div { class: crate::DIV_CENTER,
            div { class: "flex w-full flex-col rounded border p-5 shadow-md max-w-96 input-bordered",
                UserDashboard {}
            }
        }
    }
}