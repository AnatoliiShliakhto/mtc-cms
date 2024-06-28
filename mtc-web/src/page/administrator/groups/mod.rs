use std::collections::BTreeMap;

use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::group_model::GroupModel;
use mtc_model::pagination_model::PaginationModel;

use crate::APP_STATE;
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::group_handler::GroupHandler;
use crate::model::page_action::PageAction;
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
    let group_selected = use_context_provider(|| Signal::new(PageAction::None));

    let mut groups = use_context_provider(|| Signal::new(BTreeMap::<usize, GroupModel>::new()));
    let mut pagination = use_context_provider(|| Signal::new(PaginationModel::new(0, 10)));
    let mut groups_future = use_resource(move || async move { APP_STATE.peek().api.get_group_list(page()).await });

    use_effect(move || if group_selected() == PageAction::None { groups_future.restart() });

    if group_selected() == PageAction::None {
        match &*groups_future.read_unchecked() {
            Some(Ok(response)) => {
                let mut group_list = BTreeMap::<usize, GroupModel>::new();

                for (count, item) in response.data.iter().enumerate() {
                    group_list.insert(count, item.clone());
                }

                groups.set(group_list);
                pagination.set(response.pagination.clone().unwrap_or(PaginationModel::new(0, 10)));

                rsx! { GroupList { page } }
            }
            Some(Err(e)) => rsx! { ReloadingBoxComponent { message: e.message(), resource: groups_future } },
            None => rsx! { LoadingBoxComponent {} },
        }
    } else {
        rsx! { GroupSingle {} }
    }
}