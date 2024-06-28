use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::pagination_model::PaginationModel;
use mtc_model::role_model::{RoleModel, RolesModel};

use crate::APP_STATE;
use crate::component::paginator::{PaginatorComponent, PaginatorComponentMode};
use crate::handler::role_handler::RoleHandler;
use crate::model::page_action::PageAction;

#[derive(Props, Clone, PartialEq)]
pub struct RoleListProps {
    pub page: Signal<usize>,
}

#[component]
pub fn RoleList(mut props: RoleListProps) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    let mut page_action = use_context::<Signal<PageAction>>();
    let roles = use_context::<Signal<BTreeMap<usize, RoleModel>>>();
    let pagination = use_context::<Signal<PaginationModel>>();
    let mut is_busy = use_signal(|| false);

    let delete_roles = move |event: Event<FormData>| {
        event.stop_propagation();
        if let Some((&_, value)) = event.values().get_key_value("roles") {
            is_busy.set(true);
            let roles_to_delete = RolesModel { roles: value.0.to_vec().to_owned() };
            spawn(async move {
                if APP_STATE.peek().api.delete_role_list(roles_to_delete).await.is_ok() {
                    props.page.set(pagination().current_page);
                }
                is_busy.set(false);
            });
        }
    };

    rsx! {
        div { class: "flex flex-row grow",
            div { class: "flex flex-col gap-3 grow items-center p-5 body-scroll",
                form { class: "flex w-full", 
                    id: "roles-form",
                    onsubmit: delete_roles,

                    table { class: "table w-full",
                        thead {
                            tr {
                                th { style: "width: 1.75rem;" }
                                th { { translate!(i18, "messages.slug") } }
                                th { { translate!(i18, "messages.title") } }
                            }
                        }
                        tbody {
                            for (id, item) in roles.read_unchecked().to_owned() {
                                tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                    onclick: move |event| {
                                        event.stop_propagation();
                                        page_action.set(PageAction::Selected(id))
                                    },

                                    td {
                                        onclick: move |event| event.stop_propagation(),
                                        input { class: "checkbox-xs",
                                            r#type: "checkbox",
                                            name: "roles",
                                            value: item.slug.clone(),
                                        }
                                    }
                                    td { { item.slug.clone() } }
                                    td { { item.title } }
                                }
                            }
                        }
                    }
                }
                PaginatorComponent { mode: PaginatorComponentMode::Full, page: props.page, pagination }
            }
            div { class: "flex flex-col gap-3 p-5 min-w-36 body-scroll",
                if is_busy() {
                    div { class: "flex flex-col pt-4 gap-3 items-center",
                        span { class: "loading loading-bars loading-lg" }
                        span { { translate!(i18, "messages.in_progress") } }
                    }
                } else {
                    div { class: "flex flex-wrap gap-3",
                        PaginatorComponent { mode: PaginatorComponentMode::Compact, page: props.page, pagination }
                    }
                    if auth_state.is_permission("role::write") {
                        button { class: "w-full btn btn-outline btn-accent gap-3 justify-start",
                            prevent_default: "onclick",
                            onclick: move |_| page_action.set(PageAction::New),
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaSquarePlus
                            }
                            { translate!(i18, "messages.add") }
                        }
                    }
                    if auth_state.is_permission("role::delete") {
                        button { class: "w-full btn btn-outline btn-error gap-3 justify-start",
                            r#type: "submit",
                            prevent_default: "onsubmit onclick",
                            form: "roles-form",
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaTrashCan
                            }
                            { translate!(i18, "messages.delete") }
                        }
                    }
                }
            }
        }
    }
}