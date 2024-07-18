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
use crate::model::modal_model::ModalModel;
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
                match  APP_STATE.peek().api.delete_role_list(roles_to_delete).await {
                    Ok(_) => props.page.set(pagination().current_page),
                    Err(e) => APP_STATE.peek().modal.signal().set(ModalModel::Error(e.message())),
                }
                is_busy.set(false);
            });
        }
    };

    rsx! {
        section { class: "flex grow flex-row",
            div { class: "flex grow flex-col items-center gap-3 p-2 body-scroll",
                form { class: "flex w-full", 
                    id: "roles-form",
                    onsubmit: delete_roles,

                    table { class: "table w-full",
                        thead {
                            tr {
                                th { class: "w-6" }
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
            aside { class: "flex flex-col gap-3 p-2 pt-3 shadow-lg bg-base-200 min-w-48 body-scroll",
                if is_busy() {
                    div { class: "flex flex-col items-center gap-3 pt-4",
                        span { class: "loading loading-bars loading-lg" }
                        span { { translate!(i18, "messages.in_progress") } }
                    }
                } else {
                    if auth_state.is_permission("role::write") {
                        button { class: "btn btn-outline btn-accent",
                            prevent_default: "onclick",
                            onclick: move |_| page_action.set(PageAction::New),
                            Icon {
                                width: 26,
                                height: 26,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                            }
                            { translate!(i18, "messages.add") }
                        }
                    }
                    if auth_state.is_permission("role::delete") {
                        button { class: "btn btn-outline btn-error",
                            r#type: "submit",
                            prevent_default: "onsubmit onclick",
                            form: "roles-form",
                            Icon {
                                width: 18,
                                height: 18,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaTrashCan
                            }
                            { translate!(i18, "messages.delete") }
                        }
                    }
                }
                div { class: "flex grow items-end",
                    PaginatorComponent { mode: PaginatorComponentMode::Compact, page: props.page, pagination }
                }   
            }
        }
    }
}