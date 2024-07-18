use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use tracing::error;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::pagination_model::PaginationModel;
use mtc_model::user_model::{UserModel, UsersModel};

use crate::component::paginator::{PaginatorComponent, PaginatorComponentMode};
use crate::handler::user_handler::UserHandler;
use crate::model::modal_model::ModalModel;
use crate::model::page_action::PageAction;
use crate::service::user_service::UserService;
use crate::APP_STATE;

#[derive(Props, Clone, PartialEq)]
pub struct UserListProps {
    pub page: Signal<usize>,
}

#[component]
pub fn UserList(mut props: UserListProps) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    let users_details = app_state.users.signal();

    let mut page_action = use_context::<Signal<PageAction>>();
    let users = use_context::<Signal<BTreeMap<usize, UserModel>>>();
    let pagination = use_context::<Signal<PaginationModel>>();
    let mut is_busy = use_signal(|| false);
    let can_edit = use_memo(|| APP_STATE.peek().auth.peek().is_permission("user::write"));

    let delete_users = move |event: Event<FormData>| {
        event.stop_propagation();
        if let Some((&_, value)) = event.values().get_key_value("users") {
            is_busy.set(true);
            let users_to_delete = UsersModel {
                users: value.0.to_vec().to_owned(),
            };
            spawn(async move {
                match APP_STATE.peek().api.delete_user_list(users_to_delete).await {
                    Ok(_) => props.page.set(pagination().current_page),
                    Err(e) => APP_STATE
                        .peek()
                        .modal
                        .signal()
                        .set(ModalModel::Error(e.message())),
                }
                is_busy.set(false);
            });
        }
    };

    let user_block_toggle = move |login: String| {
        if !can_edit() {
            return;
        }
        spawn(async move {
            match APP_STATE.peek().api.toggle_block_user(&login).await {
                Ok(_) => (),
                Err(e) => error!("User block error: {:?}", e.message()),
            }
        });
    };

    rsx! {
        section { class: "flex grow flex-row",
            div { class: "flex grow flex-col items-center gap-3 p-2 body-scroll",
                form { class: "flex w-full",
                    id: "users-form",
                    onsubmit: delete_users,

                    table { class: "table w-full",
                        thead {
                            tr {
                                th { class: "w-6" }
                                th { { translate!(i18, "messages.login") } }
                                if !users_details().is_empty(){
                                    th { { translate!(i18, "messages.rank") } }
                                    th { { translate!(i18, "messages.name") } }
                                }
                                th { { translate!(i18, "messages.blocked") } }
                            }
                        }
                        tbody {
                            for (id, item) in users.read_unchecked().to_owned() {
                                tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                    onclick: move |event| {
                                        event.stop_propagation();
                                        page_action.set(PageAction::Selected(id))
                                    },

                                    td {
                                        onclick: move |event| event.stop_propagation(),
                                        input { class: "checkbox-xs",
                                            r#type: "checkbox",
                                            name: "users",
                                            value: item.login.clone(),
                                        }
                                    }
                                    td { { item.login.clone() } }
                                    if !users_details().is_empty(){
                                        td { { users_details().get_user_rank(&item.login) } }
                                        td { { users_details().get_user_name(&item.login) } }
                                    }
                                    td { class: "py-1",
                                        label { class: "border p-1 swap input-bordered",
                                            onclick: move |event| event.stop_propagation(),
                                            input { r#type: "checkbox",
                                                name: "blocked",
                                                checked: item.blocked,
                                                disabled: !can_edit(),
                                                onchange: move |_| user_block_toggle(item.login.clone())
                                            }
                                            div { class: "swap-on", { "❌" } }
                                            div { class: "swap-off" }
                                        }
                                    }
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
                    if can_edit() {
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
                    if auth_state.is_permission("user::delete") {
                        button { class: "btn btn-outline btn-error",
                            r#type: "submit",
                            prevent_default: "onsubmit onclick",
                            form: "users-form",
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
