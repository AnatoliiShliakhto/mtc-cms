use std::collections::{BTreeMap, BTreeSet, HashMap};

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::user_model::{UserCreateModel, UserModel, UserUpdateModel};

use crate::component::list_switcher::ListSwitcherComponent;
use crate::handler::group_handler::GroupHandler;
use crate::handler::role_handler::RoleHandler;
use crate::handler::user_handler::UserHandler;
use crate::model::modal_model::ModalModel;
use crate::model::page_action::PageAction;
use crate::service::user_service::UserService;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;

#[component]
pub fn UserSingle() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| false);

    let mut page_action = use_context::<Signal<PageAction>>();
    let users = use_context::<Signal<BTreeMap<usize, UserModel>>>();
    let users_details = app_state.users.signal();

    let user = use_memo(move || match page_action() {
        PageAction::Selected(value) => users
            .peek()
            .get(&value)
            .unwrap_or(&UserModel::default())
            .clone(),
        _ => UserModel::default(),
    });
    let mut user_form = use_signal(HashMap::<String, FormValue>::new);

    let is_new_user = use_memo(move || page_action() == PageAction::New);

    let mut user_roles = use_signal(BTreeSet::<String>::new);
    let mut all_roles = use_signal(BTreeSet::<String>::new);

    let mut user_groups = use_signal(BTreeSet::<String>::new);
    let mut all_groups = use_signal(BTreeSet::<String>::new);

    use_hook(move || {
        spawn(async move {
            is_busy.set(true);
            let mut groups_list = BTreeSet::<String>::new();
            let mut groups_user = BTreeSet::<String>::new();
            if let Ok(groups_model) = APP_STATE.peek().api.get_group_all().await {
                groups_list = groups_model
                    .groups
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<String>>();
            }
            if !is_new_user() {
                if let Ok(groups_model) = APP_STATE
                    .peek()
                    .api
                    .get_user_groups(&user().login.clone())
                    .await
                {
                    groups_user = groups_model
                        .groups
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<String>>();
                }
            }

            for group in groups_user.iter() {
                groups_list.remove(group);
            }

            all_groups.set(groups_list);
            user_groups.set(groups_user);

            let mut roles_list = BTreeSet::<String>::new();
            let mut roles_user = BTreeSet::<String>::new();
            if let Ok(roles_model) = APP_STATE.peek().api.get_role_all().await {
                roles_list = roles_model
                    .roles
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<String>>();
            }
            if !is_new_user() {
                if let Ok(roles_model) = APP_STATE
                    .peek()
                    .api
                    .get_user_roles(&user().login.clone())
                    .await
                {
                    roles_user = roles_model
                        .roles
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<String>>();
                }
            }

            for role in roles_user.iter() {
                roles_list.remove(role);
            }

            all_roles.set(roles_list);
            user_roles.set(roles_user);
            is_busy.set(false);
        });
    });

    let user_submit = move |event: Event<FormData>| {
        user_form.set(event.values());
        if !user_form.is_string_valid("login", 5)
            | (is_new_user() && !user_form.is_string_valid("password", 6))
        {
            APP_STATE
                .peek()
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            let roles = match user_roles().is_empty() {
                true => None,
                false => Some(user_roles().iter().cloned().collect::<Vec<String>>()),
            };

            let groups = match user_groups().is_empty() {
                true => None,
                false => Some(user_groups().iter().cloned().collect::<Vec<String>>()),
            };

            match match is_new_user() {
                false => {
                    app_state
                        .api
                        .update_user(
                            &user_form.get_string("login"),
                            &UserUpdateModel {
                                password: user_form.get_string_option("password"),
                                roles,
                                groups,
                                fields: None,
                            },
                        )
                        .await
                }
                true => {
                    app_state
                        .api
                        .create_user(
                            &user_form.get_string("login"),
                            &UserCreateModel {
                                password: user_form.get_string("password"),
                                roles,
                                groups,
                            },
                        )
                        .await
                }
            } {
                Ok(_) => page_action.set(PageAction::None),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }

            is_busy.set(false);
        });
    };

    let user_delete = move |_| {
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match app_state.api.delete_user(&user().login).await {
                Ok(_) => page_action.set(PageAction::None),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    rsx! {
        div { class: "flex grow select-none flex-row",
            form { class: "flex grow flex-col items-center gap-3 p-3 px-10 body-scroll",
                id: "user-form",
                prevent_default: "oninput",
                autocomplete: "off",
                oninput: move |event| user_form.set(event.values()),
                onsubmit: user_submit,
                if users_details().contains_key(&user().login) {
                    label { class: "w-full form-control",
                        div { class: "label",
                            span { class: "label-text text-primary", { translate!(i18, "messages.rank") } " / " { translate!(i18, "messages.name") }}
                        }
                        input { r#type: "text", name: "name",
                            value: [&users_details().get_user_rank(&user().login), " ", &users_details().get_user_name(&user().login)].concat(),
                            class: "input input-bordered",
                            readonly: true
                        }
                    }
                }
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary", { translate!(i18, "messages.login") } }
                    }
                    input { r#type: "text", name: "login", value: user().login.clone(),
                        class: if user_form.is_field_empty("login") | user_form.is_string_valid("login", 5) { "input input-bordered" } else { "input input-bordered input-error" },
                        readonly: !is_new_user(),
                        autofocus: is_new_user()
                    }
                    if !user_form.is_field_empty("login") && !user_form.is_string_valid("login", 5) {
                        div { class: "label",
                            span { class: "label-text-alt text-error",
                                { translate!(i18, "validate.login") }
                            }
                        }
                    }
                }
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary", { translate!(i18, "messages.password") } }
                    }
                    input { r#type: "text", name: "password",
                        class: if user_form.is_field_empty("password") | user_form.is_string_valid("password", 5) { "input input-bordered" } else { "input input-bordered input-error" },
                        autofocus: !is_new_user()
                    }
                    if !user_form.is_field_empty("password") && !user_form.is_string_valid("password", 5)  {
                        div { class: "label",
                            span { class: "label-text-alt text-error",
                                { translate!(i18, "validate.password") }
                            }
                        }
                    }
                }
                ListSwitcherComponent { title: translate!(i18, "messages.roles"), items: user_roles, all: all_roles }
                ListSwitcherComponent { title: translate!(i18, "messages.groups"), items: user_groups, all: all_groups }
            }

            div { class: "flex flex-col gap-3 p-5 shadow-lg bg-base-200 min-w-48 body-scroll",
                if is_busy() {
                    div { class: "flex flex-col items-center gap-3 pt-4",
                        span { class: "loading loading-bars loading-lg" }
                        span { { translate!(i18, "messages.in_progress") } }
                    }
                } else {
                    div { class: "mt-3 flex flex-col gap-1 rounded border p-2 input-bordered",
                        span { class: "italic label-text", { translate!(i18, "messages.created_at") } ":" }
                        span { class: "text-info", { user().created_at.format("%H:%M %d/%m/%Y").to_string() } }
                        span { class: "italic label-text", { translate!(i18, "messages.updated_at") } ":" }
                        span { class: "text-info", { user().updated_at.format("%H:%M %d/%m/%Y").to_string() } }
                    }
                    button { class: "btn btn-outline",
                        prevent_default: "onclick",
                        onclick: move |_| page_action.set(PageAction::None),
                        Icon {
                            width: 16,
                            height: 16,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::fa_regular_icons::FaCircleLeft
                        }
                        { translate!(i18, "messages.cancel") }
                    }

                    if auth_state.is_permission("user::write") {
                        button { class: "btn btn-outline btn-accent",
                            prevent_default: "onsubmit onclick",
                            r#type: "submit",
                            form: "user-form",
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaFloppyDisk
                            }
                            { translate!(i18, "messages.save") }
                        }
                    }
                    if auth_state.is_permission("user::delete") && !is_new_user() {
                        button { class: "btn btn-outline btn-error",
                            prevent_default: "onsubmit onclick",
                            onclick: user_delete,
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
