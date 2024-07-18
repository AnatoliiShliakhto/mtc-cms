use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use serde_json::{Map, Value};

use mtc_model::api_model::{ApiModel, ApiPostModel};
use mtc_model::auth_model::AuthModelTrait;
use mtc_model::field_model::FieldTypeModel;
use mtc_model::schema_model::SchemaModel;

use crate::component::loading_box::LoadingBoxComponent;
use crate::element::editor::html_field::HtmlField;
use crate::element::editor::string_field::StringField;
use crate::element::editor::text_field::TextField;
use crate::handler::content_handler::ContentHandler;
use crate::handler::schema_handler::SchemaHandler;
use crate::model::modal_model::ModalModel;
use crate::service::content_service::ContentService;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;

mod html_field;
mod string_field;
mod text_field;

#[derive(Default, Clone, PartialEq)]
pub struct ContentEdit {
    pub schema: Option<String>,
    pub api: String,
    pub is_new: bool,
}

#[derive(Props, Clone, PartialEq)]
pub struct FieldProps {
    pub slug: String,
    pub title: String,
    pub value: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct EditorProps {
    pub content: Signal<Option<ContentEdit>>,
}

pub fn Editor(mut props: EditorProps) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| true);

    let mut schema = use_signal(SchemaModel::default);
    let mut content = use_signal(ApiModel::default);
    let edit_item = props.content.peek().clone().unwrap_or_default();

    let mut content_form = use_signal(HashMap::<String, FormValue>::new);

    let mut content_published = use_signal(|| false);

    use_hook(move || {
        spawn(async move {
            let app_state = APP_STATE.peek();
            match app_state
                .api
                .get_schema(
                    match edit_item.schema {
                        Some(value) => value,
                        _ => edit_item.api.clone(),
                    }
                    .as_str(),
                )
                .await
            {
                Ok(value) => schema.set(value),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            if edit_item.is_new {
                is_busy.set(false);
                return;
            }

            if schema().is_collection {
                match app_state
                    .api
                    .get_collection_content(&schema().slug, &edit_item.api)
                    .await
                {
                    Ok(value) => {
                        content_published.set(value.published);
                        content.set(value)
                    }
                    Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
                }
            } else {
                match app_state.api.get_single_content(&schema().slug).await {
                    Ok(value) => {
                        content_published.set(value.published);
                        content.set(value)
                    }
                    Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
                }
            }

            is_busy.set(false);
        })
    });

    let schema_permission = use_memo(move || {
        if schema().is_public {
            "content".to_string()
        } else {
            schema().slug.clone()
        }
    });

    //todo SUBMIT CONTENT
    let submit_task = move |event: Event<FormData>| {
        if edit_item.is_new && !content_form.is_title_valid() | !content_form.is_slug_valid() {
            APP_STATE
                .peek()
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            return;
        }
        is_busy.set(true);

        let mut submit_fields = Map::new();
        if let Some(fields) = schema().fields {
            for field in fields.iter() {
                let mut field_value = String::new();
                if let Some((_, FormValue(value))) = event.values().get_key_value(&field.slug) {
                    if let Some(string_value) = value.first() {
                        field_value.clone_from(string_value)
                    }
                }
                submit_fields.insert(field.slug.clone(), Value::String(field_value));
            }
        }

        let submit_form = ApiPostModel {
            title: match content_form.get_string_option("title") {
                Some(value) => value,
                _ => content.read().title.clone(),
            },
            published: content_published(),
            fields: match submit_fields.is_empty() {
                true => None,
                false => Some(Value::Object(submit_fields)),
            },
        };

        let t_schema = schema().slug.clone();

        spawn(async move {
            match match edit_item.is_new {
                true => {
                    APP_STATE
                        .peek()
                        .api
                        .create_content(
                            &schema().slug,
                            &content_form.get_string("slug"),
                            &submit_form,
                        )
                        .await
                }
                false => {
                    APP_STATE
                        .peek()
                        .api
                        .update_content(
                            match &schema().is_collection {
                                true => &t_schema,
                                false => "",
                            },
                            &content.read().slug.clone(),
                            &submit_form,
                        )
                        .await
                }
            } {
                Ok(_) => props.content.set(None),
                Err(e) => APP_STATE
                    .peek()
                    .modal
                    .signal()
                    .set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    let content_delete = move |_| {
        spawn(async move {
            match APP_STATE.peek().api.delete_content(&schema().slug, &content.read().slug).await {
                Ok(_) => props.content.set(None),
                Err(e) => APP_STATE
                    .peek()
                    .modal
                    .signal()
                    .set(ModalModel::Error(e.message())),                
            }
        });
    };
    
    if is_busy() {
        return rsx! { LoadingBoxComponent {} };
    }

    rsx! {
        section { class: "flex grow select-none flex-row",
            div { class: "flex grow flex-col items-center p-3 px-10 body-scroll",
                form {
                    class: "w-full",
                    id: "content-form",
                    prevent_default: "oninput",
                    oninput: move |event| content_form.set(event.values()),
                    label { class: "w-full form-control",
                        div { class: "label",
                            span { class: "label-text text-primary", { translate!(i18, "messages.slug") } }
                        }
                        input { r#type: "text", name: "slug", value: content.read().slug.clone(),
                            class: if content_form.is_field_empty("slug") | content_form.is_slug_valid() { "input input-bordered" } else { "input input-bordered input-error" },
                            readonly: !edit_item.is_new,
                            autofocus: edit_item.is_new
                        }
                        if !content_form.is_field_empty("slug") && !content_form.is_slug_valid() {
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
                        input { r#type: "text", name: "title", value: content.read().title.clone(),
                            class: if content_form.is_field_empty("title") | content_form.is_title_valid() { "input input-bordered" } else { "input input-bordered input-error" },
                            autofocus: !edit_item.is_new
                        }
                        if !content_form.is_field_empty("title") && !content_form.is_title_valid()  {
                            div { class: "label",
                                span { class: "label-text-alt text-error",
                                    { translate!(i18, "validate.title") }
                                }
                            }
                        }
                    }
                }
                form {
                    class: "w-full",
                    id: "fields-form",
                    prevent_default: "oninput",
                    onsubmit: submit_task,
                    for field in schema().fields.unwrap_or(vec![]).iter() {
                        match field.field_type {
                            FieldTypeModel::Html => rsx! {
                                HtmlField { slug: field.slug.clone(), title: field.title.clone(), value: content.extract_string(&field.slug) }
                            },
                            FieldTypeModel::Text => rsx! {
                                TextField { slug: field.slug.clone(), title: field.title.clone(), value: content.extract_string(&field.slug) }
                            },
                            _ => rsx! {
                                StringField { slug: field.slug.clone(), title: field.title.clone(), value: content.extract_string(&field.slug) }
                            }
                        }
                    }
                }
            }
        }

        aside { class: "flex flex-col gap-3 p-2 pt-3 shadow-lg bg-base-200 min-w-48 body-scroll",
            button { class: "btn btn-outline",
                prevent_default: "onclick",
                onclick: move |_| { props.content.set(None) },
                Icon {
                    width: 22,
                    height: 22,
                    icon: dioxus_free_icons::icons::md_navigation_icons::MdArrowBack
                }
                { translate!(i18, "messages.cancel") }
            }
            div { class: "flex flex-col gap-1 rounded border p-2 input-bordered label-text",
                span { class: "italic label-text text-primary", { translate!(i18, "messages.created_at") } ":" }
                span { { content.read().created_by.clone() } }
                span { class: "label-text-alt", { content.read().created_at.clone().format("%H:%M %d/%m/%Y").to_string() } }
                span { class: "mt-1 italic label-text text-primary", { translate!(i18, "messages.updated_at") } ":" }
                span { { content.read().updated_by.clone() } }
                span { class: "label-text-alt", { content.read().updated_at.clone().format("%H:%M %d/%m/%Y").to_string() } }
            }
            label { class:
                if content_published() {
                    "items-center rounded border p-3 swap border-success text-success"
                } else {
                    "items-center rounded border p-3 swap border-warning text-warning"
                },
                input { r#type: "checkbox",
                    value: "published",
                    checked: content.read().published,
                    prevent_default: "onchange",
                    onchange: move |event| content_published.set(event.checked())
                }
                div { class: "inline-flex gap-3 swap-on",
                    Icon {
                        width: 22,
                        height: 22,
                        fill: "currentColor",
                        icon: dioxus_free_icons::icons::md_action_icons::MdVisibility
                    }
                    { translate!(i18, "messages.published") }
                }
                div { class: "inline-flex gap-3 swap-off",
                    Icon {
                        width: 22,
                        height: 22,
                        fill: "currentColor",
                        icon: dioxus_free_icons::icons::md_action_icons::MdVisibilityOff
                    }
                    { translate!(i18, "messages.draft") }
                }
            }

            if auth_state.is_permission(&[&schema_permission(), "::write"].concat()) {
                button { class: "btn btn-outline btn-accent",
                    prevent_default: "onsubmit onclick",
                    r#type: "submit",
                    form: "fields-form",
                    Icon {
                        width: 22,
                        height: 22,
                        fill: "currentColor",
                        icon: dioxus_free_icons::icons::md_content_icons::MdSave
                    }
                    { translate!(i18, "messages.save") }
                }
            }
            if !edit_item.is_new && schema.read().is_collection && auth_state.is_permission(&[&schema_permission(), "::delete"].concat()) {
                button { class: "btn btn-outline btn-error",
                    prevent_default: "onsubmit onclick",
                    onclick: content_delete,
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
