use chrono::Local;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::group_model::{GroupCreateModel, GroupModel, GroupUpdateModel};

use crate::component::breadcrumb::Breadcrumb;
use crate::component::loading_box::LoadingBoxComponent;
use crate::handler::group_handler::GroupHandler;
use crate::model::modal_model::ModalModel;
use crate::model::page_action::PageAction;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;
use crate::constants::validation::{SLUG_PATTERN, TITLE_PATTERN};

#[component]
pub fn GroupEditor() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| true);

    let mut page_action = use_context::<Signal<PageAction>>();

    let mut group = use_signal(GroupModel::default);
    let group_slug = use_memo(move || match page_action() {
        PageAction::Item(value) => value,
        _ => String::new(),
    });
    let is_new_group =
        use_memo(move || page_action().eq(&PageAction::New) | group_slug().is_empty());

    use_hook(|| {
        if is_new_group() {
            is_busy.set(false);
            return;
        }

        spawn(async move {
            match APP_STATE.peek().api.get_group(&group_slug()).await {
                Ok(value) => group.set(value),
                Err(e) => {
                    APP_STATE
                        .peek()
                        .modal
                        .signal()
                        .set(ModalModel::Error(e.message()));
                    page_action.set(PageAction::None)
                }
            }

            is_busy.set(false);
        });
    });

    let group_submit = move |event: Event<FormData>| {
        let app_state = APP_STATE.peek();
        is_busy.set(true);

        if !event.is_title_valid() | (is_new_group() & !event.is_slug_valid()) {
            app_state
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        spawn(async move {
            match match is_new_group() {
                false => {
                    app_state
                        .api
                        .update_group(
                            &group_slug(),
                            &GroupUpdateModel {
                                title: event.get_string("title"),
                            },
                        )
                        .await
                }
                true => {
                    app_state
                        .api
                        .create_group(
                            &event.get_string("slug"),
                            &GroupCreateModel {
                                title: event.get_string("title"),
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

    let group_delete = move |_| {
        let app_state = APP_STATE.read();
        is_busy.set(true);

        spawn(async move {
            match app_state.api.delete_group(&group().slug).await {
                Ok(_) => page_action.set(PageAction::None),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    if is_busy() {
        return rsx! { 
            div { class: "grid w-full place-items-center body-scroll",
                LoadingBoxComponent {} 
            }    
        };
    }

    rsx! {
        section { class: "flex grow select-none flex-row",
            form { class: "flex grow flex-col items-center gap-3 p-2 body-scroll",
                id: "group-form",
                autocomplete: "off",
                onsubmit: group_submit,
                div { class: "self-start",
                    Breadcrumb { title: translate!(i18, "messages.groups") }
                }
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary",
                            { translate!(i18, "messages.slug") }
                        }
                    }
                    input { r#type: "text", name: "slug",
                        class: "input input-bordered",
                        disabled: !is_new_group(),
                        required: true,
                        initial_value: group().slug,
                        pattern: SLUG_PATTERN,
                        title: translate!(i18, "validate.slug"),
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
                        required: true,
                        initial_value: group().title,
                        pattern: TITLE_PATTERN,
                        title: translate!(i18, "validate.title"),
                    }
                }
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
                    span { { group().created_by } }
                    span { class: "label-text-alt", { group().created_at.with_timezone(&Local).format("%H:%M %d/%m/%Y").to_string() } }
                    span { class: "mt-1 italic label-text text-primary", { translate!(i18, "messages.updated_at") } ":" }
                    span { { group().updated_by } }
                    span { class: "label-text-alt", { group().updated_at.with_timezone(&Local).format("%H:%M %d/%m/%Y").to_string() } }
                }

                if auth_state.is_permission("group::write") {
                    button { class: "btn btn-outline btn-accent",
                        r#type: "submit",
                        form: "group-form",
                        Icon {
                            width: 22,
                            height: 22,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::md_content_icons::MdSave
                        }
                        { translate!(i18, "messages.save") }
                    }
                }
                if auth_state.is_permission("group::delete") && !is_new_group() {
                    div { class: "divider" }
                    button { class: "btn btn-outline btn-error",
                        onclick: group_delete,
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
