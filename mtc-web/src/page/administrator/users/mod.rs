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
use crate::handler::user_handler::UserHandler;
use crate::page::not_found::NotFoundPage;
use crate::router::Route::UserEditorPage;
use crate::service::user_service::UserService;

pub mod editor;

#[component]
pub fn UsersPage() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    if !auth_state.is_permission("user::read") {
        return rsx! { NotFoundPage {} };
    }

    let mut breadcrumbs = app_state.breadcrumbs.signal();
    use_effect(move || {
        breadcrumbs.set(vec![
            RecordModel { title: translate!(i18, "messages.administrator"), slug: "/administrator".to_string() },
            RecordModel { title: translate!(i18, "messages.users"), slug: "/administrator/users".to_string() },
        ]);
    });

    let pagination = use_signal(|| PaginationModel::new(0, 10));
    let page = use_signal(|| 1usize);

    let users_future =
        use_resource(move || async move { APP_STATE.peek().api.get_user_list(page()).await });

    let users_details = app_state.users.signal();

    rsx! {
        match &*users_future.read() {
            Some(Ok(response)) => rsx! {
                section { class: "w-full flex-grow p-3",
                    table { class: "table w-full",
                        thead {
                            tr {
                                th { class: "w-6" }
                                th { { translate!(i18, "messages.login") } }
                                if !users_details().is_empty(){
                                    th { { translate!(i18, "messages.rank") } }
                                    th { { translate!(i18, "messages.name") } }
                                }
                            }
                        }
                        tbody {
                            for item in response.data.iter(){
                                {
                                    let m_login = item.login.clone();
                                    rsx! {
                                        tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                            onclick: move |_| { navigator().push(UserEditorPage{ user_prop: m_login.clone() }); },
                                            td { class: "text-error",
                                                if item.blocked {
                                                    Icon {
                                                        width: 16,
                                                        height: 16,
                                                        fill: "currentColor",
                                                        icon: dioxus_free_icons::icons::md_content_icons::MdBlock
                                                    }
                                                }
                                            }
                                            td { { item.login.clone() } }
                                            if !users_details().is_empty() {
                                                td { { users_details().get_user_rank(&item.login) } }
                                                td { { users_details().get_user_name(&item.login) } }
                                            }
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
                    onclick: move |_| { navigator().push(UserEditorPage{ user_prop: "new".to_string() }); },
                    Icon {
                        width: 26,
                        height: 26,
                        icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                    }
                }
            },
            Some(Err(e)) => rsx! {
                div { class: crate::DIV_CENTER,
                    ReloadingBoxComponent { message: e.message(), resource: users_future }
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
