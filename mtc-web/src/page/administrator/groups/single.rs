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
use crate::service::validator_service::ValidatorService;

#[derive(Props, Clone, PartialEq)]
pub struct GroupSingleProps {
    pub page: Signal<usize>,
    pub selected: Signal<i32>,
}

pub fn GroupSingle(mut props: GroupSingleProps) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| false);

    let mut group_form = use_signal(HashMap::<String, FormValue>::new);

    let groups = use_context::<Signal<BTreeMap<i32, GroupModel>>>();
    let selected = *props.selected.peek();
    let current_page = props.page.peek().to_owned();

    let group = match groups.peek().get_key_value(&selected) {
        Some((_, item)) => item.to_owned(),
        None => GroupModel::default(),
    };

    let group_slug = group.slug.clone();

    let group_submit = move |_| {
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match match selected {
                value if value > 0 => {
                    app_state.api.update_group(
                        &group_form.get_string("slug"),
                        &GroupUpdateModel { title: group_form.get_string("title") },
                    ).await
                }
                _ => {
                    app_state.api.create_group(
                        &group_form.get_string("slug"),
                        &GroupCreateModel { title: group_form.get_string("title") },
                    ).await
                }
            } {
                Ok(_) => {
                    props.selected.set(-1);
                    props.page.set(1)
                }
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message()))
            }

            is_busy.set(false);
        });
    };

    let group_delete = move |group: String| {
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match app_state.api.delete_group(&group).await {
                Ok(_) => {
                    props.selected.set(-1);
                    props.page.set(current_page)
                }
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message()))
            }
            is_busy.set(false);
        });
    };

    rsx! {
        div { class: "flex flex-row py-3",
            form { class: "flex flex-col gap-3 grow items-center px-10",
                id: "group-form",
                prevent_default: "oninput",
                autocomplete: "off",
                oninput: move |event| group_form.set(event.values()),
                onsubmit: group_submit,
                label { class: "form-control w-full",
                    div { class: "label",
                        span { class: "label-text", { translate!(i18, "messages.slug") } }
                    }
                    input { r#type: "text", name: "slug", value: group.slug.clone(), class: "input input-bordered", readonly: selected > 0,
                        autofocus: selected <= 0
                    }
                    if !group_form.is_field_empty("slug") && !group_form.is_slug_valid() {
                        div { class: "label",
                            span { class: "label-text-alt text-error",
                                { translate!(i18, "validate.slug") }
                            }
                        }
                    }
                }
                label { class: "form-control w-full",
                    div { class: "label",
                        span { class: "label-text", { translate!(i18, "messages.title") } }
                    }
                    input { r#type: "text", name: "title", value: group.title.clone(), class: "input input-bordered",
                        autofocus: selected > 0
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
            div { class: "flex flex-col gap-3 min-w-36",
                if is_busy() {
                    div { class: "flex flex-col pt-4 gap-3 items-center",
                        span { class: "loading loading-bars loading-lg" }
                        span { { translate!(i18, "messages.in_progress") } }
                    }
                } else {
                    div { class: "flex flex-col border input-bordered gap-1 p-2 rounded",
                        span { class: "italic label-text", { translate!(i18, "messages.created_at") } ":" }
                        span { class: "text-info", { group.created_at.format("%H:%M %d/%m/%Y").to_string() } }
                        span { class: "italic label-text", { translate!(i18, "messages.updated_at") } ":" }
                        span { class: "text-info", { group.updated_at.format("%H:%M %d/%m/%Y").to_string() } }
                    }
                    button { class: "w-full btn btn-outline gap-3 justify-start",
                        prevent_default: "onclick",
                        onclick: move |_| props.selected.set(-1),
                        Icon {
                            width: 16,
                            height: 16,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::fa_regular_icons::FaCircleLeft
                        }
                        { translate!(i18, "messages.cancel") }
                    }
                
                    if auth_state.is_permission("group::write") && group_form.is_string_valid("title", 5)
                    && (selected > 0 || group_form.is_slug_valid()) {
                        button { class: "w-full btn btn-outline btn-accent gap-3 justify-start",
                            prevent_default: "onsubmit onclick",
                            r#type: "submit",
                            form: "group-form",
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaFloppyDisk
                            }
                            { translate!(i18, "messages.save") }
                        }
                    }
                    if auth_state.is_permission("group::delete") && selected > 0 {
                        button { class: "w-full btn btn-outline btn-error gap-3 justify-start",
                            prevent_default: "onsubmit onclick",
                            onclick: move |_| group_delete(group_slug.clone()),
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