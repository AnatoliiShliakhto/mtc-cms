use std::collections::BTreeMap;

use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::pagination_model::PaginationModel;
use mtc_model::role_model::RoleModel;

use crate::APP_STATE;
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::role_handler::RoleHandler;
use crate::model::page_action::PageAction;
use crate::page::administrator::roles::list::RoleList;
use crate::page::administrator::roles::single::RoleSingle;
use crate::page::not_found::NotFoundPage;

mod list;
mod single;

#[component]
pub fn Roles() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();

    if !auth_state.is_permission("role::read") {
        return rsx! { NotFoundPage {} };
    }

    let page = use_signal(|| 1usize);
    let role_selected = use_context_provider(|| Signal::new(PageAction::None));

    let mut roles = use_context_provider(|| Signal::new(BTreeMap::<usize, RoleModel>::new()));
    let mut pagination = use_context_provider(|| Signal::new(PaginationModel::new(0, 10)));
    let mut roles_future = use_resource(move || async move { APP_STATE.peek().api.get_role_list(page()).await });

    use_effect(move || if role_selected() == PageAction::None { roles_future.restart() });

    if role_selected() == PageAction::None {
        match &*roles_future.read_unchecked() {
            Some(Ok(response)) => {
                let mut role_list = BTreeMap::<usize, RoleModel>::new();

                for (count, item) in response.data.iter().enumerate() {
                    role_list.insert(count, item.clone());
                }

                roles.set(role_list);
                pagination.set(response.pagination.clone().unwrap_or(PaginationModel::new(0, 10)));

                rsx! { RoleList { page } }
            }
            Some(Err(e)) => rsx! { ReloadingBoxComponent { message: e.message(), resource: roles_future } },
            None => rsx! { LoadingBoxComponent {} },
        }
    } else {
        rsx! { RoleSingle {} }
    }
}