use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use content::Content;
use dashboard::Dashboard;
use editor::Editor;
use groups::Groups;
use mtc_model::auth_model::AuthModelTrait;
use mtc_model::record_model::RecordModel;
use personas::Personas;
use roles::Roles;
use schema::Schema;
use users::Users;

use crate::handler::schema_handler::SchemaHandler;
use crate::page::not_found::NotFoundPage;
use crate::APP_STATE;

mod content;
mod dashboard;
mod editor;
mod groups;
mod personas;
mod roles;
mod schema;
mod users;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum AdministratorRouteModel {
    Dashboard,
    Personas,
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
    let i18 = use_i18();

    let mut active_content_api = app_state.active_content_api.signal();
    let active_content = app_state.active_content.signal();

    if !auth_state.is_permission("administrator") {
        return rsx! { NotFoundPage {} };
    }

    let mut administrator_route =
        use_context_provider(|| Signal::new(AdministratorRouteModel::Dashboard));

    let mut collections_future = use_resource(move || async {
        APP_STATE.peek().api.get_all_collections().await
    });

    rsx! {
        div { class: "flex w-full flex-row",
            aside { class: "shadow-lg bg-base-100 min-w-60 body-scroll",
                ul { class: "menu rounded",
                    if auth_state.is_permission("user::read") {
                        li {
                            a {
                                class: "inline-flex justify-between",
                                class: if administrator_route.read().eq(&AdministratorRouteModel::Personas) { "active" },
                                onclick: move |_| administrator_route.set(AdministratorRouteModel::Personas),
                                { translate!(i18, "messages.personas") }
                                Icon { class: "text-primary",
                                    width: 16,
                                    height: 16,
                                    fill: "currentColor",
                                    icon: dioxus_free_icons::icons::md_social_icons::MdGroups
                                }
                            }
                        }
                    }
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
                                                active_content_api().slug.is_empty() { "active" },
                                    onclick: move |_| {
                                        active_content_api.set(RecordModel::default());
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
                                                for item in response.list.iter() {
                                                    {
                                                        let m_item = RecordModel{ slug: item.slug.clone(), title: item.title.clone() };

                                                        rsx! {
                                                            li  {
                                                                a {
                                                                    class: if administrator_route.read().eq(&AdministratorRouteModel::Content) &&
                                                                        active_content_api().eq(&m_item) { "active" },
                                                                    onclick: move |_| {
                                                                        active_content_api.set(m_item.clone());
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
                                icon: dioxus_free_icons::icons::md_action_icons::MdAdminPanelSettings
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
                AdministratorRouteModel::Personas => rsx! { Personas {} },
                _ => rsx! { Dashboard {} },
            }
        }
    }
}
