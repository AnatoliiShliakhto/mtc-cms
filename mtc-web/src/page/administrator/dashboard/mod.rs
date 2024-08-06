use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::record_model::RecordModel;

use crate::APP_STATE;
use crate::handler::migration_handler::MigrationHandler;
use crate::model::modal_model::ModalModel;
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

    let mut migrations_future = use_resource(move || async move { APP_STATE.peek().api.get_migrations().await });

    let migrate = move |_| {
        spawn(async move {
            let app_state = APP_STATE.read();

            match app_state
                .api
                .migrate("dummy".to_string(), "dummy_password".to_string())
                .await
            {
                Ok(_) => migrations_future.restart(),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
        });
    };

    rsx! {
        section { class: "flex w-full grow flex-wrap p-3 gap-5",
            div { class: "card bg-base-100 w-full sm:w-60 shadow-md",
                div { class: "card-body",
                    h2 { class: "card-title pb-2",
                        { translate!(i18, "messages.migration") }
                    }
                    match &*migrations_future.read() {
                        Some(Ok(response)) => rsx! {
                            table { class: "table",
                                for item in response.list.iter() {
                                    tr {
                                        td { { item.clone() } }
                                    }    
                                }
                            }
                        },
                        _ => rsx! {}
                    }
                    button { class: "mt-2 w-fit self-center btn btn-primary",
                        onclick: migrate,
                        { translate!(i18, "messages.migration") }
                    }
                }    
            }
        }
    }
}
