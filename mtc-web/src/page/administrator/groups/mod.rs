use std::collections::BTreeMap;

use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::group_model::GroupModel;
use mtc_model::pagination_model::PaginationModel;

use crate::APP_STATE;
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::group_handler::GroupHandler;
use crate::page::administrator::groups::list::GroupList;
use crate::page::administrator::groups::single::GroupSingle;
use crate::page::not_found::NotFoundPage;

mod single;
mod list;

#[component]
pub fn Groups() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();

    if !auth_state.is_permission("group::read") {
        return rsx! { NotFoundPage {} };
    }

    let page = use_signal(|| 1usize);
    let group_selected = use_signal(|| -1);
    let mut groups = use_context_provider(|| Signal::new(BTreeMap::<i32, GroupModel>::new()));
    let mut pagination = use_context_provider(|| Signal::new(PaginationModel::new(0, 10)));

    let future = use_resource(move || async move { APP_STATE.peek().api.get_group_list(page()).await });

    match &*future.read_unchecked() {
        Some(Ok(response)) => {
            let mut counter = 1;
            let mut group_list = BTreeMap::<i32, GroupModel>::new();
            for item in &response.data {
                group_list.insert(counter, item.clone());
                counter += 1;
            }
            groups.set(group_list);
            pagination.set(response.pagination.clone().unwrap());

            rsx! {
                if group_selected() < 0 {
                    GroupList { page, selected: group_selected }
                } else {
                    GroupSingle { page, selected: group_selected }
                }
            }
        }
        Some(Err(e)) => rsx! { ReloadingBoxComponent { message: e.message(), resource: future } },
        None => rsx! { LoadingBoxComponent {} },
    }
}