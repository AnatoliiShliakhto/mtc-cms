use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use editor::UserEditor;
use mtc_model::pagination_model::PaginationModel;

use crate::APP_STATE;
use crate::component::breadcrumb::Breadcrumb;
use crate::component::loading_box::LoadingBoxComponent;
use crate::component::paginator::{PaginatorComponent, PaginatorComponentMode};
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::user_handler::UserHandler;
use crate::model::page_action::PageAction;
use crate::service::user_service::UserService;

mod editor;

#[component]
pub fn Users() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    let page = use_signal(|| 1usize);
    let mut page_action = use_context_provider(|| Signal::new(PageAction::None));

    let pagination = use_signal(|| PaginationModel::new(0, 10));
    let mut users_future =
        use_resource(move || async move { APP_STATE.peek().api.get_user_list(page()).await });

    let users_details = app_state.users.signal();

    use_effect(move || {
        if page_action() == PageAction::None {
            users_future.restart()
        }
    });

    if page_action().ne(&PageAction::None) {
        return rsx! { UserEditor {} };
    }

    rsx! {
        match &*users_future.read() {
            Some(Ok(response)) => rsx! {
                section { class: "flex grow flex-col items-center gap-3 p-2 body-scroll",
                    div { class: "flex w-full justify-between gap-5",
                        Breadcrumb { title: translate!(i18, "messages.users") }
                        PaginatorComponent { mode: PaginatorComponentMode::Compact, page, pagination: response.pagination.clone().unwrap_or_default() }
                    }                    
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
                                            onclick: move |_| page_action.set(PageAction::Item(m_login.clone())),
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
                    PaginatorComponent { mode: PaginatorComponentMode::Full, page, pagination: response.pagination.clone().unwrap_or_default() }
                }
                button {
                    class: "absolute right-4 bottom-4 btn btn-circle btn-neutral",
                    onclick: move |_| page_action.set(PageAction::New),
                    Icon {
                        width: 26,
                        height: 26,
                        icon: dioxus_free_icons::icons::md_content_icons::MdAdd
                    }
                }
            },
            Some(Err(e)) => rsx! { ReloadingBoxComponent { message: e.message(), resource: users_future } },
            None => rsx! { LoadingBoxComponent {} },
        }
    }
}
