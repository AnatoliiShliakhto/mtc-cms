use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::pagination_model::PaginationModel;
use mtc_model::schema_model::{SchemaModel, SchemasModel};

use crate::component::paginator::{PaginatorComponent, PaginatorComponentMode};
use crate::handler::schema_handler::SchemaHandler;
use crate::model::page_action::PageAction;
use crate::APP_STATE;

#[derive(Props, Clone, PartialEq)]
pub struct SchemaListProps {
    pub page: Signal<usize>,
}

#[component]
pub fn SchemaList(mut props: SchemaListProps) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    let mut page_action = use_context::<Signal<PageAction>>();
    let schemas = use_context::<Signal<BTreeMap<usize, SchemaModel>>>();
    let pagination = use_context::<Signal<PaginationModel>>();
    let mut is_busy = use_signal(|| false);

    let delete_schemas = move |event: Event<FormData>| {
        event.stop_propagation();
        if let Some((&_, value)) = event.values().get_key_value("schemas") {
            is_busy.set(true);
            let schemas_to_delete = SchemasModel {
                schemas: value.0.to_vec().to_owned(),
            };
            spawn(async move {
                if APP_STATE
                    .peek()
                    .api
                    .delete_schema_list(schemas_to_delete)
                    .await
                    .is_ok()
                {
                    props.page.set(pagination().current_page);
                }
                is_busy.set(false);
            });
        }
    };

    rsx! {
        div { class: "flex grow flex-row",
            div { class: "flex grow flex-col items-center gap-3 p-5 body-scroll",
                form { class: "flex w-full",
                    id: "schemas-form",
                    onsubmit: delete_schemas,

                    table { class: "table w-full",
                        thead {
                            tr {
                                th { class: "w-6" }
                                th { { translate!(i18, "messages.slug") } }
                                th { { translate!(i18, "messages.title") } }
                            }
                        }
                        tbody {
                            for (id, item) in schemas.read_unchecked().to_owned() {
                                tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                    onclick: move |event| {
                                        event.stop_propagation();
                                        if !item.is_system { page_action.set(PageAction::Selected(id)) }    
                                    },

                                    td { class: "text-nowrap",
                                        onclick: move |event| event.stop_propagation(),
                                        if item.is_system {
                                            span { { "🔒" } }    
                                        } else {
                                            input { class: "checkbox-xs",
                                                r#type: "checkbox",
                                                name: "schemas",
                                                value: item.slug.clone(),
                                            }
                                        }
                                        if item.is_collection {
                                            span { class: "pl-3 text-xl text-primary",{ "☰" } }                                            
                                        }
                                    }
                                    td { { item.slug.clone() } }
                                    td { { item.title } }
                                }
                            }
                        }
                    }
                }
                PaginatorComponent { mode: PaginatorComponentMode::Full, page: props.page, pagination }
            }
            div { class: "flex flex-col gap-3 p-5 shadow-lg bg-base-200 min-w-48 body-scroll",
                if is_busy() {
                    div { class: "flex flex-col items-center gap-3 pt-4",
                        span { class: "loading loading-bars loading-lg" }
                        span { { translate!(i18, "messages.in_progress") } }
                    }
                } else {
                    if auth_state.is_permission("schema::write") {
                        button { class: "btn btn-outline btn-accent",
                            prevent_default: "onclick",
                            onclick: move |_| page_action.set(PageAction::New),
                            Icon {
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::fa_regular_icons::FaSquarePlus
                            }
                            { translate!(i18, "messages.add") }
                        }
                    }
                    if auth_state.is_permission("schema::delete") {
                        button { class: "btn btn-outline btn-error",
                            r#type: "submit",
                            prevent_default: "onsubmit onclick",
                            form: "schemas-form",
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
                div { class: "flex grow items-end",
                    PaginatorComponent { mode: PaginatorComponentMode::Compact, page: props.page, pagination }
                }                 
            }
        }
    }
}
