use std::collections::{BTreeMap, BTreeSet};

use chrono::Local;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};

use crate::component::breadcrumb::Breadcrumb;
use crate::component::list_switcher::ListSwitcherComponent;
use crate::component::loading_box::LoadingBoxComponent;
use crate::handler::permissions_handler::PermissionsHandler;
use crate::handler::role_handler::RoleHandler;
use crate::model::modal_model::ModalModel;
use crate::page::not_found::NotFoundPage;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;

#[component]
pub fn RoleEditorPage(role: String) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    if !auth_state.is_permission("role::read") {
        return rsx! { NotFoundPage {} };
    }

    let mut is_busy = use_signal(|| true);

    let role_slug = use_memo(move || role.clone());
    let mut role = use_signal(RoleModel::default);
    let is_new_role = use_memo(move || role_slug().eq("new"));

    let mut user_access_all = use_signal(|| false);
    let mut role_permissions = use_signal(BTreeSet::<String>::new);
    let mut all_permissions = use_signal(BTreeSet::<String>::new);

    let dummy_permissions_title = use_signal(BTreeMap::<String, String>::new);

    use_hook(|| {
        spawn(async move {
            let mut permissions_list = BTreeSet::<String>::new();
            let mut permissions_role = BTreeSet::<String>::new();

            if let Ok(permissions_model) = APP_STATE.peek().api.get_permissions().await {
                permissions_list = permissions_model
                    .list
                    .iter()
                    .cloned()
                    .map(|value| value.slug)
                    .collect::<BTreeSet<String>>();
            }

            if !is_new_role() {
                match APP_STATE.peek().api.get_role(&role_slug()).await {
                    Ok(value) => {
                        user_access_all.set(value.user_access_all);
                        role.set(value)
                    }
                    Err(e) => {
                        APP_STATE
                            .peek()
                            .modal
                            .signal()
                            .set(ModalModel::Error(e.message()));
                        navigator().go_back()
                    }
                }

                if let Ok(permissions_model) = APP_STATE
                    .peek()
                    .api
                    .get_role_permissions(&role().slug.clone())
                    .await
                {
                    permissions_role = permissions_model
                        .list
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<String>>();
                }
            }

            permissions_role.iter().for_each(|permission| {
                permissions_list.remove(permission);
            });
            all_permissions.set(permissions_list);
            role_permissions.set(permissions_role);

            is_busy.set(false)
        });
    });

    let role_submit = move |event: Event<FormData>| {
        let app_state = APP_STATE.peek();
        is_busy.set(true);

        let permissions = match role_permissions().is_empty() {
            false => Some(role_permissions().iter().cloned().collect::<Vec<String>>()),
            true => None,
        };

        if !event.is_title_valid() | (is_new_role() & !event.is_slug_valid()) {
            app_state
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        spawn(async move {
            match match is_new_role() {
                false => {
                    app_state
                        .api
                        .update_role(
                            &role_slug(),
                            &RoleUpdateModel {
                                title: event.get_string("title"),
                                user_access_level: event
                                    .get_int_option("user_access_level")
                                    .unwrap_or(999),
                                user_access_all: user_access_all(),
                                permissions: permissions.clone(),
                            },
                        )
                        .await
                }
                true => {
                    app_state
                        .api
                        .create_role(
                            &event.get_string("slug"),
                            &RoleCreateModel {
                                title: event.get_string("title"),
                                user_access_level: event
                                    .get_int_option("user_access_level")
                                    .unwrap_or(999),
                                user_access_all: user_access_all(),
                                permissions: permissions.clone(),
                            },
                        )
                        .await
                }
            } {
                Ok(_) => navigator().go_back(),
                Err(e) => {
                    let role_model = RoleModel {
                        id: role().id,
                        slug: if is_new_role() {
                            event.get_string("slug")
                        } else {
                            role().slug
                        },
                        title: event.get_string("title"),
                        user_access_level: event.get_int_option("user_access_level").unwrap_or(999),
                        user_access_all: user_access_all(),
                        permissions,
                        created_at: role().created_at,
                        updated_at: role().updated_at,
                        created_by: role().created_by,
                        updated_by: role().updated_by,
                    };
                    role.set(role_model);
                    app_state.modal.signal().set(ModalModel::Error(e.message()))
                }
            }

            is_busy.set(false);
        });
    };

    let role_delete = move |_| {
        let app_state = APP_STATE.read();
        is_busy.set(true);

        spawn(async move {
            match app_state.api.delete_role(&role().slug).await {
                Ok(_) => navigator().go_back(),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    if is_busy() {
        return rsx! {
            div { class: crate::DIV_CENTER,
                LoadingBoxComponent {}
            }
        };
    }

    rsx! {
        section { class: "flex grow select-none flex-row gap-6",
            form { class: "flex grow flex-col items-center gap-3",
                id: "role-form",
                autocomplete: "off",
                onsubmit: role_submit,
                div { class: "w-full py-3",
                    Breadcrumb { title: translate!(i18, "messages.roles") }
                }
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary",
                            { translate!(i18, "messages.slug") }
                        }
                    }
                    input { r#type: "text", name: "slug",
                        class: "input input-bordered",
                        disabled: !is_new_role(),
                        minlength: 4,
                        maxlength: 30,
                        required: true,
                        initial_value: role().slug
                    }
                }
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary",
                            { translate!(i18, "messages.title") }
                        }
                    }
                    input { r#type: "text", name: "title",
                        class: "input input-bordered",
                        minlength: 4,
                        maxlength: 50,
                        required: true,
                        initial_value: role().title
                    }
                }
                div { class: "inline-flex w-full gap-5",
                    label { class: "w-fit form-control",
                        div { class: "label",
                            span { class: "label-text text-primary",
                                { translate!(i18, "messages.access_level") }
                            }
                        }
                        input { r#type: "number", name: "user_access_level",
                            class: "input input-bordered",
                            min: 1,
                            max: 999,
                            step: 1,
                            initial_value: role().user_access_level.to_string()
                        }
                    }
                    label { class: "w-fit form-control",
                        div { class: "label",
                            span { class: "label-text text-primary lowercase", { translate!(i18, "messages.users") } }
                        }
                        label { class: "w-fit rounded border px-3 py-2 swap text-warning input-bordered",
                            input {
                                r#type: "checkbox",
                                name: "is_collection",
                                checked: user_access_all(),
                                onchange: move |event| user_access_all.set(event.checked())
                            }
                            div { class: "swap-on",
                                span { class: "inline-flex flex-nowrap gap-3 items-center",
                                    Icon {
                                        width: 22,
                                        height: 22,
                                        fill: "currentColor",
                                        icon: dioxus_free_icons::icons::md_social_icons::MdGroups
                                    }
                                    { translate!(i18, "messages.access_all") }
                                }
                            }
                            div { class: "swap-off",
                                span { class: "inline-flex flex-nowrap gap-3 items-center",
                                    Icon {
                                        width: 22,
                                        height: 22,
                                        fill: "currentColor",
                                        icon: dioxus_free_icons::icons::md_social_icons::MdMilitaryTech
                                    }
                                    { translate!(i18, "messages.access_active") }
                                }
                            }
                        }
                    }
                }
                ListSwitcherComponent {
                    title: translate!(i18, "messages.permissions"),
                    items: role_permissions,
                    all: all_permissions,
                    items_title: dummy_permissions_title
                }
            }

            aside { class: "flex flex-col gap-3 pt-5 min-w-36",
                button { class: "btn btn-outline",
                    onclick: move |_| navigator().go_back(),
                    Icon {
                        width: 22,
                        height: 22,
                        icon: dioxus_free_icons::icons::md_navigation_icons::MdArrowBack
                    }
                    { translate!(i18, "messages.cancel") }
                }
                div { class: "flex flex-col gap-1 rounded border p-2 input-bordered label-text",
                    span { class: "italic label-text text-primary", { translate!(i18, "messages.created_at") } ":" }
                    span { { role().created_by } }
                    span { class: "label-text-alt", { role().created_at.with_timezone(&Local).format("%H:%M %d/%m/%Y").to_string() } }
                    span { class: "mt-1 italic label-text text-primary", { translate!(i18, "messages.updated_at") } ":" }
                    span { { role().updated_by } }
                    span { class: "label-text-alt", { role().updated_at.with_timezone(&Local).format("%H:%M %d/%m/%Y").to_string() } }
                }

                if auth_state.is_permission("role::write") {
                    button { class: "btn btn-outline btn-accent",
                        r#type: "submit",
                        form: "role-form",
                        Icon {
                            width: 22,
                            height: 22,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::md_content_icons::MdSave
                        }
                        { translate!(i18, "messages.save") }
                    }
                }
                if auth_state.is_permission("role::delete") && !is_new_role() {
                    div { class: "divider" }
                    button { class: "btn btn-outline btn-error",
                        onclick: role_delete,
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
