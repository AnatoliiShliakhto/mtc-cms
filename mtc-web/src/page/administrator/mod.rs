use std::collections::BTreeMap;

use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;
use futures_util::StreamExt;

use mtc_model::auth_model::AuthModelTrait;

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
    Content(String),
}

enum AdministratorActions {
    Update,
}

#[component]
pub fn AdministratorPage() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    if !auth_state.is_permission("administrator") {
        return rsx! { NotFoundPage {} };
    }

    let mut administrator_route =
        use_context_provider(|| Signal::new(AdministratorRouteModel::Dashboard));

    let mut collection_list = use_signal(BTreeMap::<String, String>::new);
    let mut is_updating = use_signal(|| false);
    let admin_controller = use_coroutine(
        |mut rx: UnboundedReceiver<AdministratorActions>| async move {
            while let Some(msg) = rx.next().await {
                match msg {
                    AdministratorActions::Update => {
                        if is_updating() {
                            return;
                        }
                        is_updating.set(true);
                        if let Ok(result) = APP_STATE.peek().api.get_all_collections().await {
                            let mut list = BTreeMap::<String, String>::new();
                            result.iter().for_each(|item| {
                                list.insert(item.title.clone(), item.slug.clone());
                            });
                            collection_list.set(list);
                        }

                        APP_STATE.peek().service.get_credentials();

                        is_updating.set(false);
                    }
                }
            }
        },
    );

    let mut selected_schema = use_signal(|| Option::<String>::None);

    use_hook(|| admin_controller.send(AdministratorActions::Update));

    rsx! {
        div { class: "flex w-full flex-row",
            aside { class: "shadow-lg bg-base-100 min-w-60 body-scroll",
                ul { class: "menu rounded",
                    li {
                        a { class: "inline-flex justify-between",
                            prevent_default: "onclick",
                            onclick: move |_| admin_controller.send(AdministratorActions::Update),
                            { translate!(i18, "messages.content") }
                            if is_updating() {
                                span { class: "loading loading-spinner loading-xs" }
                            } else {
                                span { class: "text-primary", "🗘" }
                            }
                        }
                        ul {
                            li {
                                a {
                                    prevent_default: "onclick",
                                    class: if administrator_route.read().eq(&AdministratorRouteModel::Content("singles".to_string())) { "active" },
                                    onclick: move |_|
                                        administrator_route.set(AdministratorRouteModel::Content("singles".to_string())),
                                    { translate!(i18, "messages.singles") }
                                }
                            }
                            li {
                                details {
                                    summary { { translate!(i18, "messages.collections") } }
                                    ul {
                                        for (title, slug) in collection_list().iter() {{
                                            let m_slug = slug.clone();
                                            rsx! {
                                                li {
                                                    a {
                                                        prevent_default: "onclick",
                                                        class: if administrator_route.read().eq(&AdministratorRouteModel::Content(slug.clone())) { "active" },
                                                        onclick: move |_|
                                                            administrator_route.set(AdministratorRouteModel::Content(m_slug.clone())),
                                                        { title.clone() }
                                                    }
                                                }
                                            }
                                        }}
                                    }
                                }
                            }
                        }
                    }

                    li { class: "pt-5",
                        a {
                            class: "inline-flex justify-between",
                            prevent_default: "onclick",
                            class: if administrator_route.read().eq(&AdministratorRouteModel::Dashboard) { "active" },
                            onclick: move |_| administrator_route.set(AdministratorRouteModel::Dashboard),
                            { translate!(i18, "messages.administrator") }
                            span { class: "text-primary",  "⌘" }
                        }
                        ul {
                            if auth_state.is_permission("schema::read") {
                                li {
                                    a {
                                        prevent_default: "onclick",
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
                                        prevent_default: "onclick",
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
                                        prevent_default: "onclick",
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
                                        prevent_default: "onclick",
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
                AdministratorRouteModel::Content(slug) => {
                    rsx! { Content { schema: selected_schema }
                        {
                            if slug.eq("singles") {
                                selected_schema.set(None)
                            } else {
                                selected_schema.set(Some(slug))
                            }
                        }
                    }
                },
                AdministratorRouteModel::Groups => rsx! { Groups {} },
                AdministratorRouteModel::Roles => rsx! { Roles {} },
                AdministratorRouteModel::Users => rsx! { Users {} },
                AdministratorRouteModel::Schema => rsx! { Schema {} },
                _ => rsx! { Dashboard {} },
            }
        }
    }
}
