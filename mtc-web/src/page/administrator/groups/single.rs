use std::collections::{BTreeMap, HashMap};

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::group_model::{GroupCreateModel, GroupModel, GroupUpdateModel};

use crate::APP_STATE;
use crate::handler::group_handler::GroupHandler;
use crate::model::modal_model::ModalModel;
use crate::model::page_action::PageAction;
use crate::service::validator_service::ValidatorService;

#[component]
pub fn GroupSingle() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| false);

    let mut page_action = use_context::<Signal<PageAction>>();
    let groups = use_context::<Signal<BTreeMap<usize, GroupModel>>>();

    let group = use_memo(move || {
        match page_action() {
            PageAction::Selected(value) => groups.peek().get(&value).unwrap_or(&GroupModel::default()).clone(),
            _ => GroupModel::default()
        }
    });
    let mut group_form = use_signal(HashMap::<String, FormValue>::new);

    let is_new_group = use_memo(move || page_action() == PageAction::New);

    let group_submit = move |event: Event<FormData>| {
        is_busy.set(true);
        group_form.set(event.values());
        if !group_form.is_string_valid("title", 5) | !group_form.is_slug_valid() {
            APP_STATE.peek().modal.signal().set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        spawn(async move {
            let app_state = APP_STATE.read();

            match match is_new_group() {
                false => {
                    app_state.api.update_group(
                        &group_form.get_string("slug"),
                        &GroupUpdateModel { title: group_form.get_string("title") },
                    ).await
                }
                true => {
                    app_state.api.create_group(
                        &group_form.get_string("slug"),
                        &GroupCreateModel { title: group_form.get_string("title") },
                    ).await
                }
            } {
                Ok(_) => page_action.set(PageAction::None),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message()))
            }

            is_busy.set(false);
        });
    };

    let group_delete = move |_| {
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match app_state.api.delete_group(&group().slug).await {
                Ok(_) => page_action.set(PageAction::None),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message()))
            }
            is_busy.set(false);
        });
    };

    rsx! {
        div { class: "flex grow select-none flex-row",
            form { class: "flex grow flex-col items-center gap-3 p-3 px-10 body-scroll",
                id: "group-form",
                prevent_default: "oninput",
                autocomplete: "off",
                oninput: move |event| group_form.set(event.values()),
                onsubmit: group_submit,
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary", { translate!(i18, "messages.slug") } }
                    }
                    input { r#type: "text", name: "slug", value: group().slug.clone(),
                        class: if group_form.is_field_empty("slug") | group_form.is_slug_valid() { "input input-bordered" } else { "input input-bordered input-error" },
                        readonly: !is_new_group(),
                        autofocus: is_new_group()
                    }
                    if !group_form.is_field_empty("slug") && !group_form.is_slug_valid() {
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
                    input { r#type: "text", name: "title", value: group().title.clone(),
                        class: if group_form.is_field_empty("title") | group_form.is_string_valid("title", 5) { "input input-bordered" } else { "input input-bordered input-error" },
                        autofocus: !is_new_group()
                    }
                    if !group_form.is_field_empty("title") && !group_form.is_string_valid("title", 5)  {
                        div { class: "label",
                            span { class: "label-text-alt text-error",
                                { translate!(i18, "validate.title") }
                            }
                        }
                    }
                }
            }
            
            div { class: "flex flex-col gap-3 p-5 shadow-lg bg-base-200 min-w-48 body-scroll",
                if is_busy() {
                    div { class: "flex flex-col items-center gap-3 pt-4",
                        span { class: "loading loading-bars loading-lg" }
                        span { { translate!(i18, "messages.in_progress") } }
                    }
                } else {
                    button { class: "btn btn-outline",
                        prevent_default: "onclick",
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
                        span { class: "label-text-alt", { group().created_at.format("%H:%M %d/%m/%Y").to_string() } }
                        span { class: "mt-1 italic label-text text-primary", { translate!(i18, "messages.updated_at") } ":" }
                        span { { group().updated_by } }
                        span { class: "label-text-alt", { group().updated_at.format("%H:%M %d/%m/%Y").to_string() } }
                    }
                
                    if auth_state.is_permission("group::write") {
                        button { class: "btn btn-outline btn-accent",
                            prevent_default: "onsubmit onclick",
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
                        button { class: "btn btn-outline btn-error",
                            prevent_default: "onsubmit onclick",
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
}