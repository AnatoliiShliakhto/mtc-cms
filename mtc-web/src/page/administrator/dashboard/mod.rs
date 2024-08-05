use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::record_model::RecordModel;

use crate::APP_STATE;
use crate::page::administrator::migration::Migration;
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
    use_effect(move || {
        breadcrumbs.set(
            vec![
                RecordModel { title: translate!(i18, "messages.administrator"), slug: "/administrator".to_string() },
            ]
        );
    });

    rsx! {
        section { class: "flex w-full grow flex-wrap p-3 gap-5",
            div { class: "card bg-base-100 min-w-sm shadow-md",
                div { class: "card-body",
                    h2 { class: "card-title pb-2",
                        { translate!(i18, "messages.migration") }
                    }
                    Migration {}
                }    
            }
        }
    }
}
