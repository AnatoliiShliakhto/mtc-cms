use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;

use crate::APP_STATE;
use crate::page::administrator::dashboard::Dashboard;
use crate::page::administrator::users::Users;
use crate::page::administrator::groups::Groups;
use crate::page::administrator::roles::Roles;
use crate::page::administrator::schema::Schema;
use crate::page::not_found::NotFoundPage;

mod dashboard;
mod groups;
mod roles;
mod users;
mod schema;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum AdministratorRouteModel {
    Dashboard,
    Groups,
    Roles,
    Users,
    Schema,
    SingleContent,
    CollectionContent,
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

    rsx! {
        div { class: "flex w-full flex-row",
            aside { class: "shadow-lg bg-base-100 min-w-60 body-scroll",
                ul { class: "menu rounded",
                    li {
                        h2 { class: "menu-title", { translate!(i18, "messages.content") } } 
                        ul {
                            li { 
                                a {
                                    prevent_default: "onclick",
                                    class: match administrator_route() { 
                                        AdministratorRouteModel::SingleContent => { "active" }, 
                                        _ => {""} 
                                    },
                                    onclick: move |_| administrator_route.set(AdministratorRouteModel::SingleContent),
                                    { translate!(i18, "messages.singles") }
                                }
                            }
                            li { 
                                a {
                                    prevent_default: "onclick",
                                    class: match administrator_route() { 
                                        AdministratorRouteModel::CollectionContent => { "active" }, 
                                        _ => {""} 
                                    },
                                    onclick: move |_| administrator_route.set(AdministratorRouteModel::CollectionContent),
                                    { translate!(i18, "messages.collections") }
                                }
                            }
                        }
                    }                        

                    li { 
                        a {
                            prevent_default: "onclick",
                            class: if administrator_route.read().eq(&AdministratorRouteModel::Dashboard) { "active" },
                            onclick: move |_| administrator_route.set(AdministratorRouteModel::Dashboard),
                            { translate!(i18, "messages.administrator") }
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
                AdministratorRouteModel::Groups => rsx! { Groups {} },
                AdministratorRouteModel::Roles => rsx! { Roles {} },
                AdministratorRouteModel::Users => rsx! { Users {} },
                AdministratorRouteModel::Schema => rsx! { Schema {} },
                _ => rsx! { Dashboard {} },
            }
        }
    }
}