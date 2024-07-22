use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::pagination_model::PaginationModel;

use crate::component::loading_box::LoadingBoxComponent;
use crate::component::paginator::{PaginatorComponent, PaginatorComponentMode};
use crate::component::reloading_box::ReloadingBoxComponent;
use crate::handler::role_handler::RoleHandler;
use crate::model::page_action::PageAction;
use crate::page::administrator::roles::editor::RoleEditor;
use crate::APP_STATE;

mod editor;

#[component]
pub fn Roles() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read_unchecked();
    let i18 = use_i18();

    let page = use_signal(|| 1usize);
    let mut page_action = use_context_provider(|| Signal::new(PageAction::None));

    let pagination = use_signal(|| PaginationModel::new(0, 10));
    let mut roles_future =
        use_resource(move || async move { APP_STATE.peek().api.get_role_list(page()).await });

    use_effect(move || {
        if page_action() == PageAction::None {
            roles_future.restart()
        }
    });

    if page_action().ne(&PageAction::None) {
        return rsx! { RoleEditor {} };
    }

    rsx! {
        match &*roles_future.read() {
            Some(Ok(response)) => rsx! {
                section { class: "flex grow flex-col items-center gap-3 p-2 body-scroll",
                    table { class: "table w-full",
                        thead {
                            tr {
                                th { { translate!(i18, "messages.slug") } }
                                th { { translate!(i18, "messages.title") } }
                            }
                        }
                        tbody {
                            for item in response.data.iter(){
                                {
                                    let m_slug = item.slug.clone();
                                    rsx! {
                                        tr { class: "cursor-pointer hover:bg-base-200 hover:shadow-md",
                                            onclick: move |_| page_action.set(PageAction::Item(m_slug.clone())),
                                            td { { item.slug.clone() } }
                                            td { { item.title.clone() } }
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
            Some(Err(e)) => rsx! { ReloadingBoxComponent { message: e.message(), resource: roles_future } },
            None => rsx! { LoadingBoxComponent {} },
        }
    }
}
