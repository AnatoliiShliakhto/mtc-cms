mod list;
mod single;

use std::collections::BTreeMap;

use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;
use mtc_model::pagination_model::PaginationModel;
use mtc_model::user_model::UserModel;

use crate::APP_STATE;
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::user_handler::UserHandler;
use crate::model::page_action::PageAction;
use crate::page::administrator::users::list::UserList;
use crate::page::administrator::users::single::UserSingle;
use crate::page::not_found::NotFoundPage;

#[component]
pub fn Users() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();

    if !auth_state.is_permission("user::read") {
        return rsx! { NotFoundPage {} };
    }

    let page = use_signal(|| 1usize);
    let user_selected = use_context_provider(|| Signal::new(PageAction::None));

    let mut users = use_context_provider(|| Signal::new(BTreeMap::<usize, UserModel>::new()));
    let mut pagination = use_context_provider(|| Signal::new(PaginationModel::new(0, 10)));
    let mut users_future = use_resource(move || async move { APP_STATE.peek().api.get_user_list(page()).await });

    use_effect(move || if user_selected() == PageAction::None { users_future.restart() });

    if user_selected() == PageAction::None {
        match &*users_future.read_unchecked() {
            Some(Ok(response)) => {
                let mut user_list = BTreeMap::<usize, UserModel>::new();

                for (count, item) in response.data.iter().enumerate() {
                    user_list.insert(count, item.clone());
                }

                users.set(user_list);
                pagination.set(response.pagination.clone().unwrap_or(PaginationModel::new(0, 10)));

                rsx! { UserList { page } }
            }
            Some(Err(e)) => rsx! { ReloadingBoxComponent { message: e.message(), resource: users_future } },
            None => rsx! { LoadingBoxComponent {} },
        }
    } else {
        rsx! { UserSingle {} }
    }
}