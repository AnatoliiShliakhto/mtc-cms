use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;

use crate::global_signal::APP_AUTH;
use crate::page::administrator::dashboard::Dashboard;
use crate::page::administrator::groups::Groups;
use crate::page::not_found::NotFoundPage;

mod dashboard;
mod groups;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub enum AdministratorRouteModel {
    Dashboard,
    Groups,
    Users,
}

#[component]
pub fn AdministratorPage() -> Element {
    if !APP_AUTH.read().is_permission("administrator") {
        return rsx! { NotFoundPage {} };
    }

    let i18 = use_i18();

    let mut administrator_route =
        use_context_provider(|| Signal::new(AdministratorRouteModel::Dashboard));

    rsx! {
        div { class: "flex flex-row w-full h-full divide-x divide-slate-400/25",
            aside { class: "bg-base-100 w-60",
                style: "scroll-behavior: smooth; scroll-padding-top: 5rem; overflow-y: auto",
                ul { class: "menu rounded",
                    li {
                        a {
                            prevent_default: "onclick",
                            class: if administrator_route.read().eq(&AdministratorRouteModel::Dashboard) { "active" },
                            onclick: move |_| { administrator_route.set(AdministratorRouteModel::Dashboard) },
                            { translate!(i18, "messages.administrator") }
                        }
                        ul {
                            li {
                                a {
                                    prevent_default: "onclick",
                                    class: if administrator_route.read().eq(&AdministratorRouteModel::Groups) { "active" },
                                    onclick: move |_| { administrator_route.set(AdministratorRouteModel::Groups) },
                                    { translate!(i18, "messages.groups") }
                                }
                            }
                        }
                    }
                    li { h5 { class: "menu-title", "Users" }
                        ul {
                            li {
                                a { "user list" }
                            }
                            li {
                                a { "add user" }
                            }
                        }
                    }
                }
            }
                div { class: "flex flex-col w-full p-2",
                match *administrator_route.read() {
                    AdministratorRouteModel::Dashboard => rsx! { Dashboard {} },
                    AdministratorRouteModel::Groups => rsx! { Groups {} },
                    _ => rsx! { Dashboard {} },
                }
            }
        }
    }
}