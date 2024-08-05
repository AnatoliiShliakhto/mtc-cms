use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::Local;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::field_model::{FieldModel, FieldTypeModel};
use mtc_model::record_model::RecordModel;
use mtc_model::schema_model::{SchemaCreateModel, SchemaModel, SchemaUpdateModel};

use crate::APP_STATE;
use crate::component::loading_box::LoadingBoxComponent;
use crate::handler::schema_handler::SchemaHandler;
use crate::model::modal_model::ModalModel;
use crate::page::not_found::NotFoundPage;
use crate::service::validator_service::ValidatorService;
use crate::constants::validation::{SLUG_PATTERN, TITLE_PATTERN};

#[component]
pub fn SchemaEditorPage(schema_prop: String) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    if !auth_state.is_permission("schema::read") {
        return rsx! { NotFoundPage {} };
    }

    let mut is_busy = use_signal(|| true);

    let mut form_is_collection = use_signal(|| false);
    let mut form_is_public = use_signal(|| false);

    let schema_slug = use_memo(move || schema_prop.clone());
    let mut schema = use_signal(SchemaModel::default);
    let is_new_schema = use_memo(move || schema_slug().eq("new"));

    let mut fields = use_signal(BTreeMap::<usize, FieldModel>::new);

    let mut breadcrumbs = app_state.breadcrumbs.signal();
    use_effect(move || {
        breadcrumbs.set(vec![
            RecordModel { title: translate!(i18, "messages.administrator"), slug: "/administrator".to_string() },
            RecordModel { title: translate!(i18, "messages.schema"), slug: "/administrator/schemas".to_string() },
            RecordModel {
                title:
                if is_new_schema() {
                    translate!(i18, "messages.add")
                } else {
                    schema().title
                }
                ,
                slug: "".to_string(),
            },
        ]);
    });

    use_effect(move || {
        if is_new_schema() {
            is_busy.set(false);
            return;
        }

        spawn(async move {
            match APP_STATE.peek().api.get_schema(&schema_slug()).await {
                Ok(value) => {
                    form_is_collection.set(value.is_collection);
                    form_is_public.set(value.is_public);

                    schema.set(value)
                }
                Err(e) => {
                    APP_STATE
                        .peek()
                        .modal
                        .signal()
                        .set(ModalModel::Error(e.message()));
                    navigator().go_back()
                }
            }
            is_busy.set(false)
        });
    });

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

    //todo check for slug duplicates
    let field_submit = move |event: Event<FormData>| {
        if !event.is_slug_valid() | !event.is_title_valid() {
            APP_STATE
                .peek()
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            return;
        };

        let new_field = FieldModel {
            slug: event.get_string("slug"),
            title: event.get_string("title"),
            field_type: FieldTypeModel::from_str(event.get_string("field_type").as_str()).unwrap(),
        };

        let id = match fields().last_key_value() {
            Some((value, _)) => *value,
            None => 0usize,
        };

        fields.try_write().unwrap().insert(id + 1, new_field);
    };

    let schema_submit = move |event: Event<FormData>| {
        let app_state = APP_STATE.peek();
        is_busy.set(true);

        if !event.is_title_valid() | (is_new_schema() & !event.is_slug_valid()) {
            app_state
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        spawn(async move {
            let is_collection = event.get_string_option("is_collection").is_some();
            let is_public = event.get_string_option("is_public").is_some();
            let field_set = match fields().is_empty() {
                true => None,
                false => Some(fields().values().cloned().collect::<Vec<FieldModel>>()),
            };

            match match is_new_schema() {
                false => {
                    app_state
                        .api
                        .update_schema(
                            &schema_slug(),
                            &SchemaUpdateModel {
                                title: event.get_string("title"),
                                fields: field_set.clone(),
                            },
                        )
                        .await
                }
                true => {
                    app_state
                        .api
                        .create_schema(
                            &event.get_string("slug"),
                            &SchemaCreateModel {
                                title: event.get_string("title"),
                                fields: field_set.clone(),
                                is_collection,
                                is_public,
                            },
                        )
                        .await
                }
            } {
                Ok(_) => navigator().go_back(),
                Err(e) => {
                    let schema_model = SchemaModel {
                        id: schema().id,
                        slug: if is_new_schema() {
                            event.get_string("slug")
                        } else {
                            schema().slug
                        },
                        title: event.get_string("title"),
                        is_system: false,
                        is_collection,
                        is_public,
                        fields: field_set,
                        created_at: schema().created_at,
                        updated_at: schema().updated_at,
                        created_by: schema().created_by,
                        updated_by: schema().updated_by,
                    };
                    schema.set(schema_model);
                    app_state.modal.signal().set(ModalModel::Error(e.message()))
                }
            }

            is_busy.set(false);
        });
    };

    let schema_delete = move |_| {
        let app_state = APP_STATE.read();
        is_busy.set(true);

        spawn(async move {
            match app_state.api.delete_schema(&schema().slug).await {
                Ok(_) => navigator().go_back(),
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

    rsx! {
        section { class: "flex grow select-none flex-row gap-6",
            div { class: "flex grow flex-col items-center gap-3",
                form { class: "w-full",
                    id: "schema-form",
                    autocomplete: "off",
                    onsubmit: schema_submit,
                    if is_new_schema() {
                        div { class: "inline-flex gap-5",
                            label { class: "w-fit form-control",
                                div { class: "label",
                                    span { class: "label-text text-primary", { translate!(i18, "messages.schema_type") } }
                                }
                                label { class: "w-fit rounded border p-3 swap text-warning input-bordered",
                                    input {
                                        r#type: "checkbox",
                                        name: "is_collection",
                                        checked: form_is_collection(),
                                        onchange: move |event| form_is_collection.set(event.checked())
                                    }
                                    div { class: "swap-on",
                                        span { class: "inline-flex flex-nowrap gap-3 items-center",
                                            Icon {
                                                width: 16,
                                                height: 16,
                                                fill: "currentColor",
                                                icon: dioxus_free_icons::icons::io_icons::IoBook
                                            }
                                            { translate!(i18, "messages.collection") }
                                        }
                                    }
                                    div { class: "swap-off",
                                        span { class: "inline-flex flex-nowrap gap-3 items-center",
                                            Icon {
                                                width: 16,
                                                height: 16,
                                                fill: "currentColor",
                                                icon: dioxus_free_icons::icons::fa_regular_icons::FaFile
                                            }
                                            { translate!(i18, "messages.single") }
                                        }
                                    }
                                }
                            }
                            label { class: "w-fit form-control",
                                div { class: "label",
                                    span { class: "label-text text-primary", { translate!(i18, "messages.access") } }
                                }
                                label { class: "w-fit rounded border p-3 swap text-warning input-bordered",
                                    input {
                                        r#type: "checkbox",
                                        name: "is_public",
                                        checked: form_is_public(),
                                        onchange: move |event| form_is_public.set(event.checked())
                                    }
                                    div { class: "swap-on",
                                        span { class: "inline-flex flex-nowrap gap-3 items-center",
                                            Icon {
                                                width: 16,
                                                height: 16,
                                                fill: "currentColor",
                                                icon: dioxus_free_icons::icons::md_action_icons::MdLockOpen
                                            }
                                            { translate!(i18, "messages.access_public") }
                                        }
                                    }
                                    div { class: "swap-off",
                                        span { class: "inline-flex flex-nowrap gap-3 items-center",
                                            Icon {
                                                width: 16,
                                                height: 16,
                                                fill: "currentColor",
                                                icon: dioxus_free_icons::icons::md_action_icons::MdLock
                                            }
                                            { translate!(i18, "messages.access_limited") }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    label { class: "w-full form-control",
                        div { class: "label",
                            span { class: "label-text text-primary",
                                { translate!(i18, "messages.slug") }
                            }
                        }
                        input { r#type: "text", name: "slug",
                            class: "input input-bordered",
                            disabled: !is_new_schema(),
                            required: true,
                            initial_value: schema().slug,
                            pattern: SLUG_PATTERN,
                            title: translate!(i18, "validate.slug")
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
                            initial_value: schema().title,
                            pattern: TITLE_PATTERN,
                            title: translate!(i18, "validate.title")
                        }
                    }
                }

                form { class: "mt-1 w-full",
                    id: "field-form",
                    autocomplete: "off",
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
                                tr { class: "hover:bg-base-200 hover:shadow-md",
                                    td {
                                        button { class: "btn btn-xs btn-ghost text-error",
                                            onclick: move |_| field_remove(&id),
                                            Icon {
                                                width: 16,
                                                height: 16,
                                                fill: "currentColor",
                                                icon: dioxus_free_icons::icons::md_navigation_icons::MdClose
                                            }
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
                    div { class: "flex flex-wrap gap-5 rounded p-2 bg-base-200",
                        select { class: "select select-bordered input-bordered",
                            name: "field_type",
                            option { value: "str", selected: true, { translate!(i18, "fields.str") } }
                            option { value: "text", { translate!(i18, "fields.text") } }
                            option { value: "html", { translate!(i18, "fields.html") } }
                        }
                        input { r#type: "text", name: "slug", placeholder: translate!(i18, "messages.slug"),
                            class: "input input-bordered",
                            required: true,
                            pattern: SLUG_PATTERN,
                            title: translate!(i18, "validate.slug"),
                        }
                        input { r#type: "text", name: "title", placeholder: translate!(i18, "messages.title"),
                            class: "min-w-72 input input-bordered",
                            required: true,
                            pattern: TITLE_PATTERN,
                            title: translate!(i18, "validate.title"),
                        }
                        button { class: "btn btn-primary",
                            r#type: "submit",
                            form: "field-form",
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                            }
                            { translate!(i18, "messages.add_field") }
                        }
                    }
                }
            }

            aside { class: "flex flex-col gap-3 pt-5 min-w-36",
                button { class: "btn btn-ghost",
                    onclick: move |_| navigator().go_back(),
                    Icon {
                        width: 22,
                        height: 22,
                        icon: dioxus_free_icons::icons::md_navigation_icons::MdArrowBack
                    }
                    { translate!(i18, "messages.cancel") }
                }
                div { class: "flex flex-col gap-1 rounded border p-2 input-bordered label-text",
                    span { class: "italic label-text text-primary", { translate!(i18, "messages.created_at") } ":" }
                    span { { schema().created_by } }
                    span { class: "label-text-alt", { schema().created_at.with_timezone(&Local).format("%H:%M %d/%m/%Y").to_string() } }
                    span { class: "mt-1 italic label-text text-primary", { translate!(i18, "messages.updated_at") } ":" }
                    span { { schema().updated_by } }
                    span { class: "label-text-alt", { schema().updated_at.with_timezone(&Local).format("%H:%M %d/%m/%Y").to_string() } }
                }

                if auth_state.is_permission("schema::write") {
                    button { class: "btn btn-primary",
                        r#type: "submit",
                        form: "schema-form",
                        Icon {
                            width: 22,
                            height: 22,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::md_content_icons::MdSave
                        }
                        { translate!(i18, "messages.save") }
                    }
                }
                if auth_state.is_permission("schema::delete") && !is_new_schema() {
                    div { class: "divider" }
                    button { class: "btn btn-ghost text-error",
                        onclick: schema_delete,
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
