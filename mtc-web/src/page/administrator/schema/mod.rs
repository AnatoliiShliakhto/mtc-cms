use std::collections::BTreeMap;

use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::pagination_model::PaginationModel;
use mtc_model::schema_model::SchemaModel;

use crate::component::loading_box::LoadingBoxComponent;
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::schema_handler::SchemaHandler;
use crate::model::page_action::PageAction;
use crate::page::administrator::schema::list::SchemaList;
use crate::page::administrator::schema::single::SchemaSingle;
use crate::page::not_found::NotFoundPage;
use crate::APP_STATE;

mod list;
mod single;

#[component]
pub fn Schema() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();

    if !auth_state.is_permission("schema::read") {
        return rsx! { NotFoundPage {} };
    }

    let page = use_signal(|| 1usize);
    let schema_selected = use_context_provider(|| Signal::new(PageAction::None));

    let mut schemas = use_context_provider(|| Signal::new(BTreeMap::<usize, SchemaModel>::new()));
    let mut pagination = use_context_provider(|| Signal::new(PaginationModel::new(0, 10)));
    let mut schema_future =
        use_resource(move || async move { APP_STATE.peek().api.get_schema_list(page()).await });

    use_effect(move || {
        if schema_selected() == PageAction::None {
            schema_future.restart()
        }
    });

    if schema_selected() == PageAction::None {
        match &*schema_future.read_unchecked() {
            Some(Ok(response)) => {
                let mut schema_list = BTreeMap::<usize, SchemaModel>::new();

                for (count, item) in response.data.iter().enumerate() {
                    schema_list.insert(count, item.clone());
                }

                schemas.set(schema_list);
                pagination.set(
                    response
                        .pagination
                        .clone()
                        .unwrap_or(PaginationModel::new(0, 10)),
                );

                rsx! { SchemaList { page } }
            }
            Some(Err(e)) => {
                rsx! { ReloadingBoxComponent { message: e.message(), resource: schema_future } }
            }
            None => rsx! { LoadingBoxComponent {} },
        }
    } else {
        rsx! { SchemaSingle {} }
    }
}
