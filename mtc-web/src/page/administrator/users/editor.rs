use std::collections::BTreeSet;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::user_model::{UserCreateModel, UserModel, UserUpdateModel};

use crate::component::list_switcher::ListSwitcherComponent;
use crate::component::loading_box::LoadingBoxComponent;
use crate::handler::group_handler::GroupHandler;
use crate::handler::role_handler::RoleHandler;
use crate::handler::user_handler::UserHandler;
use crate::model::modal_model::ModalModel;
use crate::model::page_action::PageAction;
use crate::service::user_service::UserService;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;
use crate::component::breadcrumb::Breadcrumb;

#[component]
pub fn UserEditor() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| true);

    let mut page_action = use_context::<Signal<PageAction>>();

    let mut form_login = use_signal(String::new);
    let mut form_blocked = use_signal(|| false);

    let mut user = use_signal(UserModel::default);
    let user_login = use_memo(move || match page_action() {
        PageAction::Item(value) => value,
        _ => String::new(),
    });
    let users_details = app_state.users.signal();
    let is_new_user =
        use_memo(move || page_action().eq(&PageAction::New) | user_login().is_empty());
    let mut user_roles = use_signal(BTreeSet::<String>::new);
    let mut all_roles = use_signal(BTreeSet::<String>::new);
    let mut user_groups = use_signal(BTreeSet::<String>::new);
    let mut all_groups = use_signal(BTreeSet::<String>::new);

    use_hook(|| {
        spawn(async move {
            let mut groups_list = BTreeSet::<String>::new();
            let mut groups_user = BTreeSet::<String>::new();
            let mut roles_list = BTreeSet::<String>::new();
            let mut roles_user = BTreeSet::<String>::new();

            if let Ok(groups_model) = APP_STATE.peek().api.get_group_all().await {
                groups_list = groups_model
                    .groups
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<String>>();
            }
            if let Ok(roles_model) = APP_STATE.peek().api.get_role_all().await {
                roles_list = roles_model
                    .roles
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<String>>();
            }

            if !is_new_user() {
                match APP_STATE.peek().api.get_user(&user_login()).await {
                    Ok(value) => {
                        form_login.set(value.login.clone());
                        form_blocked.set(value.blocked);

                        user.set(value)
                    },
                    Err(e) => {
                        APP_STATE
                            .peek()
                            .modal
                            .signal()
                            .set(ModalModel::Error(e.message()));
                        page_action.set(PageAction::None)
                    }
                }

                if let Ok(groups_model) = APP_STATE.peek().api.get_user_groups(&user_login()).await
                {
                    groups_user = groups_model
                        .groups
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<String>>();
                }

                if let Ok(roles_model) = APP_STATE.peek().api.get_user_roles(&user_login()).await {
                    roles_user = roles_model
                        .roles
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<String>>();
                }
            }

            groups_user.iter().for_each(|group| {
                groups_list.remove(group);
            });
            roles_user.iter().for_each(|role| {
                roles_list.remove(role);
            });

            all_groups.set(groups_list);
            user_groups.set(groups_user);

            all_roles.set(roles_list);
            user_roles.set(roles_user);

            is_busy.set(false)
        });
    });

    let user_submit = move |event: Event<FormData>| {
        let app_state = APP_STATE.peek();
        is_busy.set(true);

        let roles = match user_roles().is_empty() {
            true => None,
            false => Some(user_roles().iter().cloned().collect::<Vec<String>>()),
        };

        let groups = match user_groups().is_empty() {
            true => None,
            false => Some(user_groups().iter().cloned().collect::<Vec<String>>()),
        };

        if is_new_user() & !event.is_login_valid() {
            app_state
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        spawn(async move {
            match match is_new_user() {
                false => {
                    app_state
                        .api
                        .update_user(
                            &user_login(),
                            &UserUpdateModel {
                                blocked: event.get_string_option("blocked").is_some(),
                                password: event.get_string_option("password"),
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
                            &event.get_string("login"),
                            &UserCreateModel {
                                blocked: event.get_string_option("blocked").is_some(),
                                password: event.get_string("password"),
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
        let app_state = APP_STATE.read();
        is_busy.set(true);

        spawn(async move {
            match app_state.api.delete_user(&user_login()).await {
                Ok(_) => page_action.set(PageAction::None),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    if is_busy() {
        return rsx! { LoadingBoxComponent {} };
    }

    rsx! {
        section { class: "flex grow select-none flex-row",
            form { class: "flex grow flex-col items-center gap-3 p-2 body-scroll",
                id: "user-form",
                autocomplete: "off",
                onsubmit: user_submit,
                div { class: "p-1 self-start",
                    Breadcrumb { title: translate!(i18, "messages.users") }
                }                   
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary",
                            { translate!(i18, "messages.login") }
                        }
                    }
                    input { r#type: "text", name: "login",
                        class: "input input-bordered",
                        disabled: !is_new_user(),
                        minlength: 5,
                        maxlength: 15,
                        required: true,
                        value: form_login(),
                        oninput: move |event| form_login.set(event.value())
                    }
                }
                if users_details().contains_key(&user().login) {
                    label { class: "w-full form-control",
                        div { class: "label",
                            span { class: "label-text text-primary", { translate!(i18, "messages.rank") } " / " { translate!(i18, "messages.name") }}
                        }
                        input { r#type: "text", name: "name",
                            value: [&users_details().get_user_rank(&user().login), " ", &users_details().get_user_name(&user().login)].concat(),
                            class: "input input-bordered",
                            disabled: true
                        }
                    }
                }
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary",
                            { translate!(i18, "messages.password") }
                        }
                    }
                    input { r#type: "password", name: "password",
                        class: "input input-bordered",
                        minlength: 6,
                        maxlength: 15,
                    }
                }
                ListSwitcherComponent { title: translate!(i18, "messages.roles"), items: user_roles, all: all_roles }
                ListSwitcherComponent { title: translate!(i18, "messages.groups"), items: user_groups, all: all_groups }
            }

            //todo blocked
            aside { class: "flex flex-col gap-3 p-2 pt-3 shadow-lg bg-base-200 min-w-48 body-scroll",
                button { class: "btn btn-outline",
                    onclick: move |_| page_action.set(PageAction::None),
                    Icon {
                        width: 22,
                        height: 22,
                        icon: dioxus_free_icons::icons::md_navigation_icons::MdArrowBack
                    }
                    { translate!(i18, "messages.cancel") }
                }
                div { class: "flex flex-col gap-1 rounded border p-2 input-bordered label-text",
                    span { class: "italic label-text text-primary", { translate!(i18, "messages.created_at") } ":" }
                    span { { user().created_by } }
                    span { class: "label-text-alt", { user().created_at.format("%H:%M %d/%m/%Y").to_string() } }
                    span { class: "mt-1 italic label-text text-primary", { translate!(i18, "messages.updated_at") } ":" }
                    span { { user().updated_by } }
                    span { class: "label-text-alt", { user().updated_at.format("%H:%M %d/%m/%Y").to_string() } }
                }

                label { class:
                    if form_blocked() {
                        "items-center rounded border p-3 swap border-error text-error"
                    } else {
                        "items-center rounded border p-3 swap border-success text-success"
                    },
                    input { r#type: "checkbox",
                        name: "blocked",
                        form: "user-form",
                        checked: form_blocked(),
                        onchange: move |event| form_blocked.set(event.checked())
                    }
                    div { class: "inline-flex gap-3 swap-on",
                        Icon {
                            width: 22,
                            height: 22,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::md_content_icons::MdBlock
                        }
                        { translate!(i18, "messages.user_blocked") }
                    }
                    div { class: "inline-flex gap-3 swap-off",
                        Icon {
                            width: 22,
                            height: 22,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::md_action_icons::MdVerifiedUser
                        }
                        { translate!(i18, "messages.user_active") }
                    }
                }


                if auth_state.is_permission("user::write") {
                    button { class: "btn btn-outline btn-accent",
                        r#type: "submit",
                        form: "user-form",
                        Icon {
                            width: 22,
                            height: 22,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::md_content_icons::MdSave
                        }
                        { translate!(i18, "messages.save") }
                    }
                }
                if auth_state.is_permission("user::delete") && !is_new_user() {
                    div { class: "divider" }
                    button { class: "btn btn-outline btn-error",
                        onclick: user_delete,
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
        }
    }
}
