use std::collections::BTreeSet;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::role_model::{RoleCreateModel, RoleModel, RoleUpdateModel};

use crate::component::list_switcher::ListSwitcherComponent;
use crate::component::loading_box::LoadingBoxComponent;
use crate::handler::permissions_handler::PermissionsHandler;
use crate::handler::role_handler::RoleHandler;
use crate::model::modal_model::ModalModel;
use crate::model::page_action::PageAction;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;
use crate::component::breadcrumb::Breadcrumb;

#[component]
pub fn RoleEditor() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| true);

    let mut page_action = use_context::<Signal<PageAction>>();

    let mut form_slug = use_signal(String::new);
    let mut form_title = use_signal(String::new);

    let mut role = use_signal(RoleModel::default);
    let role_slug = use_memo(move || match page_action() {
        PageAction::Item(value) => value,
        _ => String::new(),
    });
    let is_new_role = use_memo(move || page_action().eq(&PageAction::New) | role_slug().is_empty());
    let mut role_permissions = use_signal(BTreeSet::<String>::new);
    let mut all_permissions = use_signal(BTreeSet::<String>::new);

    use_hook(|| {
        spawn(async move {
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
                match APP_STATE.peek().api.get_role(&role_slug()).await {
                    Ok(value) => {
                        form_slug.set(value.slug.clone());
                        form_title.set(value.title.clone());
                        
                        role.set(value)
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
                                permissions,
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
        let app_state = APP_STATE.read();
        is_busy.set(true);

        spawn(async move {
            match app_state.api.delete_role(&role().slug).await {
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
                id: "role-form",
                autocomplete: "off",
                onsubmit: role_submit,
                div { class: "p-1 self-start",
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
                        value: form_slug(),
                        oninput: move |event| form_slug.set(event.value()) 
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
                        value: form_title(),
                        oninput: move |event| form_title.set(event.value()) 
                    }
                }
                ListSwitcherComponent { title: translate!(i18, "messages.permissions"), items: role_permissions, all: all_permissions }
            }

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
                    span { { role().created_by } }
                    span { class: "label-text-alt", { role().created_at.format("%H:%M %d/%m/%Y").to_string() } }
                    span { class: "mt-1 italic label-text text-primary", { translate!(i18, "messages.updated_at") } ":" }
                    span { { role().updated_by } }
                    span { class: "label-text-alt", { role().updated_at.format("%H:%M %d/%m/%Y").to_string() } }
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
