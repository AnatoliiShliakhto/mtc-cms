use chrono::Local;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::permission_model::PermissionDtoModel;
use mtc_model::record_model::RecordModel;

use crate::APP_STATE;
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::permissions_handler::PermissionsHandler;
use crate::model::modal_model::ModalModel;
use crate::page::not_found::NotFoundPage;
use crate::service::validator_service::ValidatorService;

pub fn PermissionsPage() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| false);
    let mut is_create_dialog_shown = use_signal(|| false);

    if !auth_state.is_permission("role::read") {
        return rsx! { NotFoundPage {} };
    }

    let mut breadcrumbs = app_state.breadcrumbs.signal();
    use_effect(move || {
        breadcrumbs.set(vec![
            RecordModel { title: translate!(i18, "messages.administrator"), slug: "/administrator".to_string() },
            RecordModel { title: translate!(i18, "messages.permissions"), slug: "/administrator/permissions".to_string() },
        ]);
    });

    let mut permissions_future =
        use_resource(move || async move { APP_STATE.peek().api.get_custom_permissions().await });

    let create_permission = move |event: Event<FormData>| {
        let app_state = APP_STATE.peek();
        is_busy.set(true);

        if !event.is_slug_valid() {
            app_state
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        spawn(async move {
            match app_state.api.create_custom_permission(&PermissionDtoModel { slug: event.get_string("slug") }).await {
                Ok(_) => { 
                    is_create_dialog_shown.set(false);
                    permissions_future.restart()
                },
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    let mut remove_permission = move |slug: String| {
        let app_state = APP_STATE.read();
        is_busy.set(true);

        spawn(async move {
            match app_state.api.delete_custom_permission(&PermissionDtoModel { slug }).await {
                Ok(_) => permissions_future.restart(),
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

    if is_create_dialog_shown() {
        return rsx! {
            section { class: "flex grow select-none flex-row gap-6",
                form { class: "flex grow flex-col items-center gap-3",
                    id: "permission-form",
                    autocomplete: "off",
                    onsubmit: create_permission,

                    label { class: "w-full form-control",
                        div { class: "label",
                            span { class: "label-text text-primary",
                                { translate!(i18, "messages.slug") }
                            }
                        }
                        input { r#type: "text", name: "slug",
                            class: "input input-bordered",
                            minlength: 4,
                            maxlength: 30,
                            required: true,
                        }
                    }
                    div { class: "flex p-2 gap-5 flex-inline",
                        button { class: "btn btn-primary",
                            r#type: "submit",
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                            }
                            { translate!(i18, "messages.add") }
                        }
                        button { class: "btn btn-neutral",
                            onclick: move |_| is_create_dialog_shown.set(false),
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_navigation_icons::MdCancel
                            }
                            { translate!(i18, "messages.cancel") }
                        }
                    }
                }
            }
        };
    }

    rsx! {
        match &*permissions_future.read() {
            Some(Ok(response)) => rsx! {
                section { class: "w-full flex-grow p-3",
                    table { class: "table w-full",
                        thead {
                            tr {
                                if auth_state.is_permission("role::write") {
                                    th { class: "w-6" }    
                                }
                                th { { translate!(i18, "messages.slug") } }
                                th { { translate!(i18, "messages.created_by") } }
                                th { { translate!(i18, "messages.created_at") } }
                            }
                        }
                        tbody {
                            for item in response.iter(){
                                {
                                    let m_slug = item.slug.clone();
                                    rsx! {
                                        tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                            onclick: move |_| { },
                                            if auth_state.is_permission("role::write") {
                                                td {
                                                    button { class: "btn btn-xs btn-ghost text-error",
                                                        onclick: move |_| remove_permission(m_slug.clone()),
                                                        Icon {
                                                            width: 16,
                                                            height: 16,
                                                            fill: "currentColor",
                                                            icon: dioxus_free_icons::icons::md_navigation_icons::MdClose
                                                        }
                                                    }                                                
                                                }
                                            }
                                            td { { item.slug.clone() } }
                                            td { { item.created_by.clone() } }
                                            td { { item.created_at.with_timezone(&Local).format("%H:%M %d/%m/%Y").to_string() } }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if auth_state.is_permission("role::write") {
                    button {
                        class: "fixed right-4 bottom-4 btn btn-circle btn-neutral",
                        onclick: move |_| is_create_dialog_shown.set(true),
                        Icon {
                            width: 26,
                            height: 26,
                            icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                        }
                    }
                }    
            },
            Some(Err(e)) => rsx! {
                div { class: crate::DIV_CENTER,
                    ReloadingBoxComponent { message: e.message(), resource: permissions_future }
                }
            },
            None => rsx! {
                div { class: crate::DIV_CENTER,
                    LoadingBoxComponent {}
                }
            },
        }
    }
}