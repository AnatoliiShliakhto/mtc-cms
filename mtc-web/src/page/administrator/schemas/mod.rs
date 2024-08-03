use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::pagination_model::PaginationModel;
use mtc_model::record_model::RecordModel;

use crate::APP_STATE;
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::paginator::{PaginatorComponent, PaginatorComponentMode};
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::schema_handler::SchemaHandler;
use crate::page::not_found::NotFoundPage;
use crate::router::Route::SchemaEditorPage;

pub mod editor;

#[component]
pub fn SchemasPage() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    if !auth_state.is_permission("schema::read") {
        return rsx! { NotFoundPage {} };
    }

    let mut breadcrumbs = app_state.breadcrumbs.signal();
    breadcrumbs.set(vec![
        RecordModel { title: translate!(i18, "messages.administrator"), slug: "/administrator".to_string() },
        RecordModel { title: translate!(i18, "messages.schema"), slug: "/administrator/schemas".to_string() },
    ]);

    let page = use_signal(|| 1usize);
    let pagination = use_signal(|| PaginationModel::new(0, 10));

    let schemas_future =
        use_resource(move || async move { APP_STATE.peek().api.get_schema_list(page()).await });

    rsx! {
        match &*schemas_future.read() {
            Some(Ok(response)) => rsx! {
                section { class: "w-full flex-grow p-3",
                    table { class: "table w-full",
                        thead {
                            tr {
                                th { class: "w-6" }
                                th { { translate!(i18, "messages.slug") } }
                                th { { translate!(i18, "messages.title") } }
                            }
                        }
                        tbody {
                            for item in response.data.iter(){
                                {
                                    let m_slug = item.slug.clone();
                                    let m_can_view = !item.is_system;
                                    rsx! {
                                        tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                            onclick: move |_| {
                                                if m_can_view {
                                                    navigator().push(SchemaEditorPage{ schema_prop: m_slug.clone() });
                                                }
                                            },
                                            td { class: "text-primary",
                                                if item.is_system {
                                                    Icon {
                                                        width: 16,
                                                        height: 16,
                                                        fill: "currentColor",
                                                        icon: dioxus_free_icons::icons::md_action_icons::MdLock
                                                    }
                                                } else if item.is_collection {
                                                    Icon {
                                                        width: 16,
                                                        height: 16,
                                                        fill: "currentColor",
                                                        icon: dioxus_free_icons::icons::io_icons::IoBook
                                                    }
                                                } else {
                                                    Icon {
                                                        width: 16,
                                                        height: 16,
                                                        fill: "currentColor",
                                                        icon: dioxus_free_icons::icons::fa_regular_icons::FaFile
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
                    div { class: "flex w-full py-2 justify-center",
                        PaginatorComponent { mode: PaginatorComponentMode::Full, page, pagination: response.pagination.clone().unwrap_or_default() }
                    }    
                }
                button {
                    class: "fixed right-4 bottom-4 btn btn-circle btn-neutral",
                    onclick: move |_| { navigator().push(SchemaEditorPage{ schema_prop: "new".to_string() }); },
                    Icon {
                        width: 26,
                        height: 26,
                        icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                    }
                }
            },
            Some(Err(e)) => rsx! {
                div { class: crate::DIV_CENTER,
                    ReloadingBoxComponent { message: e.message(), resource: schemas_future }
                }
            },
            None => rsx! {
                div { class: crate::DIV_CENTER,
                    LoadingBoxComponent {}
                }
            },
        }
    }
}
