use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;

use crate::APP_STATE;
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
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();
    let i18 = use_i18();

    if !auth_state.is_permission("administrator") {
        return rsx! { NotFoundPage {} };
    }

    let mut administrator_route =
        use_context_provider(|| Signal::new(AdministratorRouteModel::Dashboard));

    rsx! {
        div { class: "flex flex-row divide-x divide-slate-400/25 grow",
            aside { class: "bg-base-100 min-w-60 body-scroll",
                ul { class: "menu rounded",
                    li { 
                        a {
                            prevent_default: "onclick",
                            class: if administrator_route.read().eq(&AdministratorRouteModel::Dashboard) { "active" },
                            onclick: move |_| administrator_route.set(AdministratorRouteModel::Dashboard),
                            { translate!(i18, "messages.administrator") }
                        }
                        ul {
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
                    }
                }
            }
            div { class: "flex flex-col p-2 grow body-scroll",
                match administrator_route() {
                    AdministratorRouteModel::Groups => rsx! { Groups {} },
                    _ => rsx! { Dashboard {} },
                }
            }
        }
    }
}