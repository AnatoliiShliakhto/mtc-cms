use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaUser;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;

use crate::APP_STATE;
use crate::router::Route::{AdministratorPage, DashboardPage};
use crate::service::auth_service::AuthService;

#[component]
pub fn AccountControllerComponent() -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read().clone();
    let i18 = use_i18();

    rsx! {
        if !auth_state.is_auth() {
            Link { class: "btn btn-ghost join-item",
                to: DashboardPage {},
                Icon {
                    width: 20,
                    height: 20,
                    fill: "currentColor",
                    icon: FaUser
                }
            }
        } else {
            div { class: "dropdown dropdown-end dropdown-hover join-item",
                div { tabindex: "0", role: "button", class: "btn btn-ghost join-item",
                    Icon {
                        width: 20,
                        fill: "green",
                        icon: FaUser
                    }
                }
                ul { tabindex: "0", class: "dropdown-content z-[1] menu p-2 shadow-md bg-base-100 w-52 border input-bordered rounded",
                    "onclick": "document.activeElement.blur()",
                    li { Link { to: DashboardPage {}, { translate!(i18, "messages.dashboard") } } }
                    if auth_state.is_admin() {
                        li { Link { to: AdministratorPage {}, { translate!(i18, "messages.administrator") } } }
                    }
                    div { class: "divider my-0" }
                    li {
                        a {
                            onclick: move |_| app_state.service.sign_out(),
                            { translate!(i18, "messages.sign_out") }
                        }
                    }
                }
            }
        }
    }
}