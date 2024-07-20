use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;

use crate::component::loading_box::LoadingBoxComponent;
use crate::element::editor::Editor;
use crate::handler::schema_handler::SchemaHandler;
use crate::page::administrator::content::Content;
use crate::page::administrator::dashboard::Dashboard;
use crate::page::administrator::groups::Groups;
use crate::page::administrator::roles::Roles;
use crate::page::administrator::schema::Schema;
use crate::page::administrator::users::Users;
use crate::page::not_found::NotFoundPage;
use crate::service::health_service::HealthService;
use crate::APP_STATE;

mod content;
mod dashboard;
mod groups;
mod roles;
mod schema;
mod users;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum AdministratorRouteModel {
    Dashboard,
    Groups,
    Roles,
    Users,
    Schema,
    Content,
    ContentEditor,
}

#[component]
pub fn AdministratorPage() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let is_busy = app_state.is_busy.signal();
    let i18 = use_i18();

    let mut active_content_api = app_state.active_content_api.signal();
    let active_content = app_state.active_content.signal();

    if !auth_state.is_permission("administrator") {
        return rsx! { NotFoundPage {} };
    }

    let mut administrator_route =
        use_context_provider(|| Signal::new(AdministratorRouteModel::Dashboard));

    let mut collections_future = use_resource(move || async {
        APP_STATE.peek().service.get_credentials();

        APP_STATE.peek().api.get_all_collections().await
    });

    rsx! {
        if is_busy() {
            LoadingBoxComponent {}
        }
        div { class: "flex w-full flex-row",
            aside { class: "shadow-lg bg-base-100 min-w-60 body-scroll",
                ul { class: "menu rounded",
                    li {
                        a { class: "inline-flex justify-between",
                            onclick: move |_| collections_future.restart(),
                            { translate!(i18, "messages.content") },
                            Icon { class: "text-primary",
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_navigation_icons::MdRefresh
                            }
                        }
                        ul {
                            li {
                                a {
                                    class: if administrator_route.read().eq(&AdministratorRouteModel::Content) &&
                                                active_content_api().is_empty() { "active" },
                                    onclick: move |_| {
                                        active_content_api.set(String::new());
                                        administrator_route.set(AdministratorRouteModel::Content);
                                    },
                                    { translate!(i18, "messages.singles") }
                                }
                            }
                            li {
                                details {
                                    summary { { translate!(i18, "messages.collections") } }
                                    ul {
                                        match &*collections_future.read() {
                                            Some(Ok(response)) => rsx! {
                                                for item in response.iter() {
                                                    {
                                                        let m_slug = item.slug.clone();
                                                        rsx! {
                                                            li  {
                                                                a {
                                                                    class: if administrator_route.read().eq(&AdministratorRouteModel::Content) &&
                                                                        active_content_api().eq(&m_slug) { "active" },
                                                                    onclick: move |_| {
                                                                        active_content_api.set(m_slug.clone());
                                                                        administrator_route.set(AdministratorRouteModel::Content);
                                                                    },
                                                                    { item.title.clone() }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            },
                                            Some(Err(_)) => rsx! {
                                                li {
                                                    a {
                                                        onclick: move |_| collections_future.restart(),
                                                        div { class: "inline-flex items-center gap-3",
                                                            Icon {
                                                                width: 16,
                                                                height: 16,
                                                                fill: "currentColor",
                                                                icon: dioxus_free_icons::icons::md_navigation_icons::MdRefresh
                                                            }
                                                            span { { translate!(i18, "messages.refresh") } }
                                                        }
                                                    }
                                                }
                                            },
                                            None => rsx! {
                                                li {
                                                    div { class: "inline-flex items-center gap-3",
                                                        span { class: "loading loading-spinner loading-xs" }
                                                        span { { translate!(i18, "messages.loading") } }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    li { class: "pt-5",
                        a {
                            class: "inline-flex justify-between",
                            class: if administrator_route.read().eq(&AdministratorRouteModel::Dashboard) { "active" },
                            onclick: move |_| administrator_route.set(AdministratorRouteModel::Dashboard),
                            { translate!(i18, "messages.administrator") }
                            Icon { class: "text-primary",
                                width: 16,
                                height: 16,
                                fill: "currentColor",
                                icon: dioxus_free_icons::icons::md_social_icons::MdGroups
                            }
                        }
                        ul {
                            if auth_state.is_permission("schema::read") {
                                li {
                                    a {
                                        class: match administrator_route() {
                                            AdministratorRouteModel::Schema => { "active" },
                                            _ => {""}
                                        },
                                        onclick: move |_| administrator_route.set(AdministratorRouteModel::Schema),
                                        { translate!(i18, "messages.schema") }
                                    }
                                }
                            }
                            if auth_state.is_permission("group::read") {
                                li {
                                    a {
                                        class: match administrator_route() {
                                            AdministratorRouteModel::Groups => { "active" },
                                            _ => {""}
                                        },
                                        onclick: move |_| administrator_route.set(AdministratorRouteModel::Groups),
                                        { translate!(i18, "messages.groups") }
                                    }
                                }
                            }
                            if auth_state.is_permission("role::read") {
                                li {
                                    a {
                                        class: match administrator_route() {
                                            AdministratorRouteModel::Roles => { "active" },
                                            _ => {""}
                                        },
                                        onclick: move |_| administrator_route.set(AdministratorRouteModel::Roles),
                                        { translate!(i18, "messages.roles") }
                                    }
                                }
                            }
                            if auth_state.is_permission("user::read") {
                                li {
                                    a {
                                        class: match administrator_route() {
                                            AdministratorRouteModel::Users => { "active" },
                                            _ => {""}
                                        },
                                        onclick: move |_| administrator_route.set(AdministratorRouteModel::Users),
                                        { translate!(i18, "messages.users") }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            match administrator_route() {
                AdministratorRouteModel::ContentEditor => rsx! { Editor {} },
                AdministratorRouteModel::Content => rsx! { Content {} },
                AdministratorRouteModel::Groups => rsx! { Groups {} },
                AdministratorRouteModel::Roles => rsx! { Roles {} },
                AdministratorRouteModel::Users => rsx! { Users {} },
                AdministratorRouteModel::Schema => rsx! { Schema {} },
                _ => rsx! { Dashboard {} },
            }
        }
    }
}
