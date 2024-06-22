use std::collections::{BTreeMap, HashMap};

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::group_model::{GroupCreateModel, GroupModel};

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

    let mut group_submit = use_signal(HashMap::<String, FormValue>::new);

    let groups = use_context::<Signal<BTreeMap<i32, GroupModel>>>();
    let selected = *props.selected.peek();

    let group = match groups.peek().get_key_value(&selected) {
        Some((_, item)) => item.to_owned(),
        None => GroupModel::default(),
    };

    let submit_task = move |_| {
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match app_state.api.create_group(
                &group_submit.get_string("slug"),
                &GroupCreateModel { title: group_submit.get_string("title") },
            ).await {
                Ok(_) => {
                    props.selected.set(-1);
                    props.page.set(1)
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
                oninput: move |event| group_submit.set(event.values()),
                onsubmit: submit_task,
                label { class: "form-control w-full",
                    input { r#type: "text", name: "slug", value: group.slug.clone(), class: "input input-bordered",
                        placeholder: translate!(i18, "messages.slug"),
                        disabled: selected > 0,
                    }
                    if !group_submit.is_field_empty("slug") && !group_submit.is_slug_valid() {
                        div { class: "label",
                            span { class: "label-text-alt text-error",
                                { translate!(i18, "validate.slug") }
                            }
                        }
                    }
                }
                label { class: "form-control w-full",
                    input { r#type: "text", name: "title", value: group.title.clone(), class: "input input-bordered",
                        placeholder: translate!(i18, "messages.title")
                    }
                    if !group_submit.is_field_empty("title") && !group_submit.is_string_valid("title", 5)  {
                        div { class: "label",
                            span { class: "label-text-alt text-error",
                                { translate!(i18, "validate.title") }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col gap-3 min-w-36",
                div { class: "flex flex-col border input-bordered gap-1 p-2 rounded",
                    span { class: "italic", { translate!(i18, "messages.created_at") } ":" }
                    span { class: "text-info", { group.created_at.format("%H:%M %d/%m/%Y").to_string() } }
                    span { class: "italic", { translate!(i18, "messages.updated_at") } ":" }
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
                
                if auth_state.is_permission("group::write") && !is_busy() && group_submit.is_string_valid("title", 5)
                    && (selected > 0 || group_submit.is_slug_valid()) {
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
                if auth_state.is_permission("group::delete") && selected > 0 && !is_busy() {
                    button { class: "w-full btn btn-outline btn-error gap-3 justify-start",
                        prevent_default: "onsubmit onclick",
                        Icon {
                            width: 16,
                            height: 16,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::fa_regular_icons::FaTrashCan
                        }
                        { translate!(i18, "messages.delete") }
                    }
                }
                if is_busy() {
                    span { class: "loading loading-bars loading-lg p-3 self-center" }
                }
            }
        }
    }
}