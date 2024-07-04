use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::field_model::{FieldModel, FieldTypeModel};
use mtc_model::schema_model::{SchemaCreateModel, SchemaModel, SchemaUpdateModel};

use crate::handler::schema_handler::SchemaHandler;
use crate::model::modal_model::ModalModel;
use crate::model::page_action::PageAction;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;

#[component]
pub fn SchemaSingle() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut is_busy = use_signal(|| false);

    let mut page_action = use_context::<Signal<PageAction>>();
    let schemas = use_context::<Signal<BTreeMap<usize, SchemaModel>>>();

    let schema = use_memo(move || match page_action() {
        PageAction::Selected(value) => schemas
            .peek()
            .get(&value)
            .unwrap_or(&SchemaModel::default())
            .clone(),
        _ => SchemaModel::default(),
    });
    let mut schema_form = use_signal(HashMap::<String, FormValue>::new);
    let mut field_form = use_signal(HashMap::<String, FormValue>::new);

    let is_new_schema = use_memo(move || page_action() == PageAction::New);

    let mut fields = use_signal(BTreeMap::<usize, FieldModel>::new);

    use_effect(move || {
        let schema_fields = match schema().fields {
            Some(items) => {
                let mut result = BTreeMap::<usize, FieldModel>::new();
                for (count, item) in items.iter().enumerate() {
                    result.insert(count, item.clone());
                }
                result
            }
            None => BTreeMap::<usize, FieldModel>::new(),
        };
        fields.set(schema_fields);
    });

    let mut field_remove = move |item: &usize| {
        fields.try_write().unwrap().remove(item);
    };

    let field_submit = move |event: Event<FormData>| {
        field_form.set(event.values());

        if !field_form.is_string_valid("title", 5) | !field_form.is_slug_valid() {
            APP_STATE
                .peek()
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        //todo check for slug duplicates
        let new_field = FieldModel {
            slug: field_form.get_string("slug"),
            title: field_form.get_string("title"),
            field_type: FieldTypeModel::from_str(field_form.get_string("field_type").as_str())
                .unwrap(),
        };

        let id = match fields().last_key_value() {
            Some((value, _)) => *value,
            None => 0usize,
        };

        fields.try_write().unwrap().insert(id + 1, new_field);
    };

    let schema_submit = move |event: Event<FormData>| {
        schema_form.set(event.values());
        if !schema_form.is_string_valid("title", 5) | !schema_form.is_slug_valid() {
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
            let is_collection = schema_form.get_string_option("is_collection").is_some();
            let field_set = match fields().is_empty() {
                true => None,
                false => Some(fields().values().cloned().collect::<Vec<FieldModel>>()),
            };

            match match is_new_schema() {
                false => {
                    app_state
                        .api
                        .update_schema(
                            &schema_form.get_string("slug"),
                            &SchemaUpdateModel {
                                title: schema_form.get_string("title"),
                                fields: field_set,
                            },
                        )
                        .await
                }
                true => {
                    app_state
                        .api
                        .create_schema(
                            &schema_form.get_string("slug"),
                            &SchemaCreateModel {
                                title: schema_form.get_string("title"),
                                fields: field_set,
                                is_collection,
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

    let schema_delete = move |_| {
        spawn(async move {
            is_busy.set(true);
            let app_state = APP_STATE.read();

            match app_state.api.delete_schema(&schema().slug).await {
                Ok(_) => page_action.set(PageAction::None),
                Err(e) => app_state.modal.signal().set(ModalModel::Error(e.message())),
            }
            is_busy.set(false);
        });
    };

    rsx! {
        div { class: "flex grow select-none flex-row",
            div { class: "flex grow flex-col items-center p-3 px-10 body-scroll",
                form { class: "w-full",
                    id: "schema-form",
                    prevent_default: "oninput",
                    autocomplete: "off",
                    oninput: move |event| schema_form.set(event.values()),
                    onsubmit: schema_submit,
                    if is_new_schema() {
                        label { class: "w-full form-control",
                            div { class: "label",
                                span { class: "label-text text-primary", { translate!(i18, "messages.schema_type") } }
                            }
                            label { class: "w-fit rounded border px-3 py-1 swap text-warning input-bordered",
                                input { r#type: "checkbox", name: "is_collection", value: true }
                                div { class: "swap-on whitespace-pre", "☰   " { translate!(i18, "messages.collection") } }
                                div { class: "swap-off whitespace-pre", "⚊   " { translate!(i18, "messages.single") } }
                            }
                        }
                    }
                    label { class: "w-full form-control",
                        div { class: "label",
                            span { class: "label-text text-primary", { translate!(i18, "messages.slug") } }
                        }
                        input { r#type: "text", name: "slug", value: schema().slug.clone(),
                            class: if schema_form.is_field_empty("slug") | schema_form.is_slug_valid() { "input input-bordered" } else { "input input-bordered input-error" },
                            readonly: !is_new_schema(),
                            autofocus: is_new_schema()
                        }
                        if !schema_form.is_field_empty("slug") && !schema_form.is_slug_valid() {
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
                        input { r#type: "text", name: "title", value: schema().title.clone(),
                            class: if schema_form.is_field_empty("title") | schema_form.is_string_valid("title", 5) { "input input-bordered" } else { "input input-bordered input-error" },
                            autofocus: !is_new_schema()
                        }
                        if !schema_form.is_field_empty("title") && !schema_form.is_string_valid("title", 5)  {
                            div { class: "label",
                                span { class: "label-text-alt text-error",
                                    { translate!(i18, "validate.title") }
                                }
                            }
                        }
                    }
                }

                form { class: "mt-1 w-full",
                    id: "field-form",
                    prevent_default: "oninput",
                    autocomplete: "off",
                    oninput: move |event| field_form.set(event.values()),
                    onsubmit: field_submit,
                    div { class: "label",
                        span { class: "label-text text-primary", "⌘ " { translate!(i18, "messages.fields") } }
                    }
                    table { class: "table w-full",
                        thead {
                            tr {
                                th { class: "w-6" }
                                th { { translate!(i18, "messages.type") } }
                                th { { translate!(i18, "messages.slug") } }
                                th { { translate!(i18, "messages.title") } }
                            }
                        }
                        tbody {
                            for (id, field) in fields() {
                                tr {
                                    td {
                                        button { class: "btn btn-xs btn-ghost",
                                            prevent_default: "onclick",
                                            onclick: move |_| field_remove(&id),
                                            "❌"
                                        }
                                    }
                                    td { { translate!(i18, ["fields.", field.field_type.to_string().as_str()].concat().as_str()) } }
                                    td { { field.slug.clone() } }
                                    td { { field.title.clone() } }
                                }
                            }
                        }
                    }
                    div { class: "mt-1 label",
                        span { class: "label-text text-primary", "⌘ " { translate!(i18, "messages.new_field") } }
                    }
                    div { class: "flex flex-wrap gap-5 rounded p-3 bg-base-200",
                        select { class: "select select-bordered input-bordered",
                            name: "field_type",
                            option { value: "str", selected: true, { translate!(i18, "fields.str") } }
                            option { value: "text", { translate!(i18, "fields.text") } }
                            option { value: "html", { translate!(i18, "fields.html") } }
                            option { value: "bool", { translate!(i18, "fields.bool") } }
                            option { value: "int", { translate!(i18, "fields.int") } }
                            option { value: "float", { translate!(i18, "fields.float") } }
                            option { value: "datetime", { translate!(i18, "fields.datetime") } }
                        }
                        input { r#type: "text", name: "slug", placeholder: translate!(i18, "messages.slug"),
                            class: if field_form.is_field_empty("slug") | field_form.is_slug_valid() { "input input-bordered" } else { "input input-bordered input-error" }
                        }
                        input { r#type: "text", name: "title", placeholder: translate!(i18, "messages.title"),
                            class: if field_form.is_field_empty("title") | field_form.is_string_valid("title", 5) { "min-w-72 input input-bordered" } else { "min-w-72 input input-bordered input-error" }
                        }
                        button { class: "btn btn-outline btn-accent",
                            prevent_default: "onsubmit onclick",
                            r#type: "submit",
                            form: "field-form",
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaSquarePlus
                            }
                            { translate!(i18, "messages.add_field") }
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
                    div { class: "flex flex-col gap-1 rounded border p-2 input-bordered label-text",
                        span { class: "italic label-text text-primary", { translate!(i18, "messages.created_at") } ":" }
                        span { { schema().created_by } }
                        span { class: "label-text-alt", { schema().created_at.format("%H:%M %d/%m/%Y").to_string() } }
                        span { class: "mt-1 italic label-text text-primary", { translate!(i18, "messages.updated_at") } ":" }
                        span { { schema().updated_by } }
                        span { class: "label-text-alt", { schema().updated_at.format("%H:%M %d/%m/%Y").to_string() } }
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

                    if auth_state.is_permission("schema::write") {
                        button { class: "btn btn-outline btn-accent",
                            prevent_default: "onsubmit onclick",
                            r#type: "submit",
                            form: "schema-form",
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaFloppyDisk
                            }
                            { translate!(i18, "messages.save") }
                        }
                    }
                    if auth_state.is_permission("schema::delete") && !is_new_schema() {
                        button { class: "btn btn-outline btn-error",
                            prevent_default: "onsubmit onclick",
                            onclick: schema_delete,
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
