use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use serde_json::{Map, Value};

use html_field::HtmlField;
use mtc_model::api_model::{ApiModel, ApiPostModel};
use mtc_model::auth_model::AuthModelTrait;
use mtc_model::field_model::FieldTypeModel;
use mtc_model::schema_model::SchemaModel;
use string_field::StringField;
use text_field::TextField;

use crate::component::loading_box::LoadingBoxComponent;
use crate::handler::content_handler::ContentHandler;
use crate::handler::schema_handler::SchemaHandler;
use crate::model::modal_model::ModalModel;
use crate::page::administrator::AdministratorRouteModel;
use crate::service::content_service::ContentService;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;
use crate::component::breadcrumb::Breadcrumb;

mod html_field;
mod string_field;
mod text_field;

#[derive(Props, Clone, PartialEq)]
pub struct FieldProps {
    pub slug: String,
    pub title: String,
    pub value: String,
}

pub fn Editor() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();
    let mut is_busy = use_signal(|| true);

    let mut administrator_route = use_context::<Signal<AdministratorRouteModel>>();

    let mut form_slug = use_signal(String::new);
    let mut form_title = use_signal(String::new);
    
    let active_content_api = app_state.active_content_api.signal();
    let active_content = app_state.active_content.signal();
    let is_new = use_memo(move || active_content().slug.is_empty());

    let mut schema = use_signal(SchemaModel::default);
    let mut content = use_signal(ApiModel::default);
    let mut form_published = use_signal(|| false);

    use_hook(|| {
        let app_state = APP_STATE.peek();

        let mut api_schema = active_content().slug;
        if !active_content_api().slug.is_empty() {
            api_schema = active_content_api().slug;
        }

        spawn(async move {
            match APP_STATE.peek().api.get_schema(&api_schema).await {
                Ok(value) => schema.set(value),
                Err(e) => {
                    app_state.modal.signal().set(ModalModel::Error(e.message()));
                    administrator_route.set(AdministratorRouteModel::Content)
                }
            }

            if active_content().slug.is_empty() {
                is_busy.set(false);
                return;
            }

            if schema().is_collection {
                match app_state
                    .api
                    .get_collection_content(&schema().slug, &active_content().slug)
                    .await
                {
                    Ok(value) => {
                        form_slug.set(value.slug.clone());
                        form_title.set(value.title.clone());
                        form_published.set(value.published);
                        
                        content.set(value)
                    }
                    Err(e) => {
                        app_state.modal.signal().set(ModalModel::Error(e.message()));
                        administrator_route.set(AdministratorRouteModel::Content)
                    }
                }
            } else {
                match app_state.api.get_single_content(&schema().slug).await {
                    Ok(value) => {
                        form_slug.set(value.slug.clone());
                        form_title.set(value.title.clone());
                        
                        form_published.set(value.published);
                        content.set(value)
                    }
                    Err(e) => {
                        app_state.modal.signal().set(ModalModel::Error(e.message()));
                        administrator_route.set(AdministratorRouteModel::Content)
                    }
                }
            }
            is_busy.set(false);
        });
    });

    let schema_permission = use_memo(move || {
        if schema().is_public {
            "content".to_string()
        } else {
            schema().slug.clone()
        }
    });

    let submit_task = move |event: Event<FormData>| {
        if !event.is_title_valid() | (is_new() & !event.is_slug_valid()) {
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
            fields.iter().for_each(|field| {
                submit_fields.insert(
                    field.slug.clone(),
                    Value::String(event.get_string(&field.slug)),
                );
            });
        }

        let submit_form = ApiPostModel {
            title: event.get_string("title"),
            published: event.get_string_option("published").is_some(),
            fields: match submit_fields.is_empty() {
                true => None,
                false => Some(Value::Object(submit_fields)),
            },
        };

        let t_schema = schema().slug.clone();

        spawn(async move {
            match match is_new() {
                true => {
                    APP_STATE
                        .peek()
                        .api
                        .create_content(&schema().slug, &event.get_string("slug"), &submit_form)
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
                Ok(_) => administrator_route.set(AdministratorRouteModel::Content),
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
            match APP_STATE
                .peek()
                .api
                .delete_content(&schema().slug, &content.read().slug)
                .await
            {
                Ok(_) => administrator_route.set(AdministratorRouteModel::Content),
                Err(e) => APP_STATE
                    .peek()
                    .modal
                    .signal()
                    .set(ModalModel::Error(e.message())),
            }
        });
    };

    if is_busy() {
        return rsx! {
            LoadingBoxComponent {}
        };
    }
    
    rsx! {
        section { class: "flex grow select-none flex-row",
            form { class: "flex grow flex-col items-center p-3 px-10 body-scroll",
                id: "content-form",
                autocomplete: "off",
                onsubmit: submit_task,
                div { class: "p-1 self-start",
                    Breadcrumb { title:
                        if active_content_api().slug.is_empty() {
                            translate!(i18, "messages.singles")
                        } else {    
                            schema().title
                        }
                    }
                }  
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary", { translate!(i18, "messages.slug") } }
                    }
                    input { r#type: "text", name: "slug",
                        class: "input input-bordered",
                        disabled: !is_new(),
                        minlength: 4,
                        maxlength: 30,
                        required: true,
                        value: form_slug(),
                        oninput: move |event| form_slug.set(event.value()) 
                    }
                }
                label { class: "w-full form-control",
                    div { class: "label",
                        span { class: "label-text text-primary", { translate!(i18, "messages.title") } }
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

        aside { class: "flex flex-col gap-3 p-2 pt-3 shadow-lg bg-base-200 min-w-48 body-scroll",
            button { class: "btn btn-outline",
                onclick: move |_| administrator_route.set(AdministratorRouteModel::Content),
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
                if form_published() {
                    "items-center rounded border p-3 swap border-success text-success"
                } else {
                    "items-center rounded border p-3 swap border-warning text-warning"
                },
                input { r#type: "checkbox",
                    name: "published",
                    form: "content-form",
                    checked: form_published(),
                    onchange: move |event| form_published.set(event.checked())
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
                    r#type: "submit",
                    form: "content-form",
                    Icon {
                        width: 22,
                        height: 22,
                        fill: "currentColor",
                        icon: dioxus_free_icons::icons::md_content_icons::MdSave
                    }
                    { translate!(i18, "messages.save") }
                }
            }
            if !is_new() && schema.read().is_collection && auth_state.is_permission(&[&schema_permission(), "::delete"].concat()) {
                div { class: "divider" }
                button { class: "btn btn-outline btn-error",
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
