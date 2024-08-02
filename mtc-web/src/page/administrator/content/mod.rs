use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::api_model::{ApiListItemModel, ApiModel, ApiPostModel};
use mtc_model::auth_model::AuthModelTrait;
use mtc_model::schema_model::SchemaModel;

use crate::component::breadcrumb::Breadcrumb;
use crate::component::loading_box::LoadingBoxComponent;
use crate::handler::content_handler::ContentHandler;
use crate::handler::schema_handler::SchemaHandler;
use crate::model::modal_model::ModalModel;
use crate::page::not_found::NotFoundPage;
use crate::router::Route::EditorPage;
use crate::service::validator_service::ValidatorService;
use crate::APP_STATE;

#[component]
pub fn ContentPage(schema_prop: String) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    let mut content_list = use_signal(BTreeMap::<String, ApiListItemModel>::new);
    let mut schema_slug = use_signal(|| "singles".to_string());
    let mut schema = use_signal(SchemaModel::default);
    let mut content = use_signal(ApiModel::default);
    let mut is_busy = use_signal(|| true);
    let mut is_new_content = use_signal(|| false);

    if !auth_state.is_permission("writer") | !auth_state.is_permission("schema::read") {
        return rsx! { NotFoundPage {} };
    }

    use_effect(use_reactive((&schema_prop,), move |(schema_prop,)| {
        is_busy.set(true);

        spawn(async move {
            schema_slug.set(schema_prop);
            is_new_content.set(false);

            if schema_slug().ne("singles") {
                match APP_STATE.peek().api.get_schema(&schema_slug()).await {
                    Ok(value) => schema.set(value),
                    Err(e) => {
                        APP_STATE
                            .peek()
                            .modal
                            .signal()
                            .set(ModalModel::Error(e.message()));
                        navigator().go_back()
                    }
                }
            }

            match APP_STATE.peek().api.get_content_list(&schema_slug()).await {
                Ok(value) => {
                    let list = value
                        .iter()
                        .map(|item| (item.title.clone(), item.clone()))
                        .collect::<BTreeMap<String, ApiListItemModel>>();
                    content_list.set(list);
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
    }));

    let schema_permission = use_memo(move || {
        if schema().is_public {
            "content::write".to_string()
        } else {
            [schema().slug.clone().as_str(), "::write"].concat()
        }
    });

    let content_submit = move |event: Event<FormData>| {
        let app_state = APP_STATE.peek();
        is_busy.set(true);

        if !event.is_title_valid() | !event.is_slug_valid() {
            app_state
                .modal
                .signal()
                .set(ModalModel::Error(translate!(i18, "errors.fields")));
            is_busy.set(false);
            return;
        };

        spawn(async move {
            match app_state
                .api
                .create_content(
                    &schema().slug,
                    event.get_string("slug").as_str(),
                    &ApiPostModel {
                        title: event.get_string("title"),
                        published: false,
                        fields: None,
                    },
                )
                .await
            {
                Ok(_) => {
                    navigator().push(EditorPage {
                        schema_prop: schema_slug(),
                        content_prop: event.get_string("slug").clone(),
                    });
                }
                Err(e) => {
                    let content_model = ApiModel {
                        id: "".to_string(),
                        slug: event.get_string("slug"),
                        title: event.get_string("title"),
                        fields: None,
                        published: false,
                        created_at: Default::default(),
                        updated_at: Default::default(),
                        created_by: "".to_string(),
                        updated_by: "".to_string(),
                    };
                    content.set(content_model);
                    app_state.modal.signal().set(ModalModel::Error(e.message()))
                }
            }

            is_busy.set(false)
        });
    };

    if is_busy() {
        return rsx! {
            div { class: crate::DIV_CENTER,
                LoadingBoxComponent {}
            }
        };
    }

    if is_new_content() {
        return rsx! {
            section { class: "flex grow select-none flex-row gap-6",
                form { class: "flex grow flex-col items-center gap-3",
                    id: "content-form",
                    autocomplete: "off",
                    onsubmit: content_submit,
                    div { class: "w-full py-3",
                        Breadcrumb { title:
                            if schema_slug().eq("singles") {
                                translate!(i18, "messages.singles")
                            } else {
                                schema().title.clone()
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
                            minlength: 4,
                            maxlength: 30,
                            required: true,
                            initial_value: content.read().slug.clone()
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
                            initial_value: content.read().title.clone()
                        }
                    }
                    div { class: "flex p-2 gap-5 flex-inline",
                        button { class: "btn btn-outline btn-success",
                            r#type: "submit",
                            Icon {
                                width: 24,
                                height: 24,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                            }
                            { translate!(i18, "messages.add") }
                        }
                        button { class: "btn btn-outline",
                            onclick: move |_| is_new_content.set(false),
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
        section { class: "w-full flex-grow p-3",
            div { class: "w-full py-3",
                Breadcrumb { title:
                    if schema_slug().eq("singles") {
                        translate!(i18, "messages.singles")
                    } else {
                        schema().title.clone()
                    }
                }
            }

            table { class: "table w-full",
                thead {
                    tr {
                        th { class: "w-6" }
                        th { { translate!(i18, "messages.slug") } }
                        th { { translate!(i18, "messages.title") } }
                    }
                }
                tbody {
                    for (_, item) in content_list().iter() {
                        {
                            let item = item.clone();
                            rsx! {
                                tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                    onclick: move |_| { navigator().push(EditorPage{ schema_prop: schema_slug(), content_prop: item.slug.clone() }); },
                                    td {
                                        if !item.published {
                                            Icon { class: "text-warning",
                                                width: 16,
                                                height: 16,
                                                fill: "currentColor",
                                                icon: dioxus_free_icons::icons::md_action_icons::MdVisibilityOff
                                            }
                                        }
                                    }
                                    td { { item.slug.clone() } }
                                    td { { item.title.clone() } }
                                }
                            }
                        }
                    }
                }
            }

            if schema_slug().ne("singles") && auth_state.is_permission(&schema_permission()) {
                button {
                    class: "fixed right-4 bottom-4 btn btn-circle btn-neutral",
                    onclick: move |_| is_new_content.set(true),
                    Icon {
                        width: 26,
                        height: 26,
                        icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                    }
                }
            }
        }
    }
}
