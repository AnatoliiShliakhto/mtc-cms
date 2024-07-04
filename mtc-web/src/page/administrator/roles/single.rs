use std::collections::{BTreeMap, BTreeSet, HashMap};

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};

use crate::component::list_switcher::ListSwitcherComponent;
use crate::handler::permissions_handler::PermissionsHandler;
use crate::handler::role_handler::RoleHandler;
use crate::model::modal_model::ModalModel;
use crate::model::page_action::PageAction;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;

#[component]
pub fn RoleSingle() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| false);

    let mut page_action = use_context::<Signal<PageAction>>();
    let roles = use_context::<Signal<BTreeMap<usize, RoleModel>>>();

    let role = use_memo(move || match page_action() {
        PageAction::Selected(value) => roles
            .peek()
            .get(&value)
            .unwrap_or(&RoleModel::default())
            .clone(),
        _ => RoleModel::default(),
    });
    let mut role_form = use_signal(HashMap::<String, FormValue>::new);

    let is_new_role = use_memo(move || page_action() == PageAction::New);

    let mut role_permissions = use_signal(BTreeSet::<String>::new);
    let mut all_permissions = use_signal(BTreeSet::<String>::new);

    use_hook(move || {
        spawn(async move {
            is_busy.set(true);
            let mut permissions_list = BTreeSet::<String>::new();
            let mut permissions_role = BTreeSet::<String>::new();
            if let Ok(permissions_model) = APP_STATE.peek().api.get_permissions().await {
                permissions_list = permissions_model
                    .permissions
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<String>>();
            }
            if !is_new_role() {
                if let Ok(permissions_model) = APP_STATE
                    .peek()
                    .api
                    .get_role_permissions(&role().slug.clone())
                    .await
                {
                    permissions_role = permissions_model
                        .permissions
                        .iter()
                        .cloned()
                        .collect::<BTreeSet<String>>();
                }
            }

            for permission in permissions_role.iter() {
                permissions_list.remove(permission);
            }

            all_permissions.set(permissions_list);
            role_permissions.set(permissions_role);
            is_busy.set(false);
        });
    });

    let role_submit = move |event: Event<FormData>| {
        role_form.set(event.values());
        if !role_form.is_string_valid("title", 5) | !role_form.is_slug_valid() {
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

            let permissions = match role_permissions().is_empty() {
                true => None,
                false => Some(role_permissions().iter().cloned().collect::<Vec<String>>()),
            };

            match match is_new_role() {
                false => {
                    app_state
                        .api
                        .update_role(
                            &role_form.get_string("slug"),
                            &RoleUpdateModel {
                                title: role_form.get_string("title"),
                                permissions,
                            },
                        )
                        .await
                }
                true => {
                    app_state
                        .api
                        .create_role(
                            &role_form.get_string("slug"),
                            &RoleCreateModel {
                                title: role_form.get_string("title"),
                                permissions,
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

    let role_delete = move |_| {
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match app_state.api.delete_role(&role().slug).await {
                Ok(_) => page_action.set(PageAction::None),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    rsx! {
        div { class: "flex grow select-none flex-row",
            form { class: "flex grow flex-col items-center p-3 px-10 body-scroll",
                id: "role-form",
                prevent_default: "oninput",
                autocomplete: "off",
                oninput: move |event| role_form.set(event.values()),
                onsubmit: role_submit,
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary", { translate!(i18, "messages.slug") } }
                    }
                    input { r#type: "text", name: "slug", value: role().slug.clone(),
                        class: if role_form.is_field_empty("slug") | role_form.is_slug_valid() { "input input-bordered" } else { "input input-bordered input-error" },
                        readonly: !is_new_role(),
                        autofocus: is_new_role()
                    }
                    if !role_form.is_field_empty("slug") && !role_form.is_slug_valid() {
                        div { class: "label",
                            span { class: "label-text-alt text-error",
                                { translate!(i18, "validate.slug") }
                            }
                        }
                    }
                }
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary", { translate!(i18, "messages.title") } }
                    }
                    input { r#type: "text", name: "title", value: role().title.clone(),
                        class: if role_form.is_field_empty("title") | role_form.is_string_valid("title", 5) { "input input-bordered" } else { "input input-bordered input-error" },
                        autofocus: !is_new_role()
                    }
                    if !role_form.is_field_empty("title") && !role_form.is_string_valid("title", 5)  {
                        div { class: "label",
                            span { class: "label-text-alt text-error",
                                { translate!(i18, "validate.title") }
                            }
                        }
                    }
                }
                ListSwitcherComponent { title: translate!(i18, "messages.permissions"), items: role_permissions, all: all_permissions }
            }

            div { class: "flex flex-col gap-3 p-5 shadow-lg bg-base-200 min-w-48 body-scroll",
                if is_busy() {
                    div { class: "flex flex-col items-center gap-3 pt-4",
                        span { class: "loading loading-bars loading-lg" }
                        span { { translate!(i18, "messages.in_progress") } }
                    }
                } else {
                    div { class: "flex flex-col gap-1 rounded border p-2 input-bordered label-text",
                        span { class: "italic label-text text-primary", { translate!(i18, "messages.created_at") } ":" }
                        span { { role().created_by } }
                        span { class: "label-text-alt", { role().created_at.format("%H:%M %d/%m/%Y").to_string() } }
                        span { class: "mt-1 italic label-text text-primary", { translate!(i18, "messages.updated_at") } ":" }
                        span { { role().updated_by } }
                        span { class: "label-text-alt", { role().updated_at.format("%H:%M %d/%m/%Y").to_string() } }
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

                    if auth_state.is_permission("role::write") {
                        button { class: "btn btn-outline btn-accent",
                            prevent_default: "onsubmit onclick",
                            r#type: "submit",
                            form: "role-form",
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaFloppyDisk
                            }
                            { translate!(i18, "messages.save") }
                        }
                    }
                    if auth_state.is_permission("role::delete") && !is_new_role() {
                        button { class: "btn btn-outline btn-error",
                            prevent_default: "onsubmit onclick",
                            onclick: role_delete,
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
