use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaUser;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;

use crate::action::auth_action::AuthAction;
use crate::global_signal::APP_AUTH;
use crate::router::Route::{AdministratorPage, DashboardPage};

#[component]
pub fn AccountControllerComponent() -> Element {
    let i18 = use_i18();

    rsx! {
        if !APP_AUTH().is_auth() {
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
                ul { tabindex: "0", class: "dropdown-content z-[2] menu p-2 shadow-md bg-base-100 w-52 border input-bordered rounded",
                    "onclick": "document.activeElement.blur()",
                    li { Link { to: DashboardPage {}, { translate!(i18, "messages.dashboard") } } }
                    if APP_AUTH().is_admin() {
                        li { Link { to: AdministratorPage {}, { translate!(i18, "messages.administrator") } } }
                    }
                    div { class: "divider my-0" }
                    li {
                        a {
                            onclick: move |_| {
                                spawn(async move {
                                    use_coroutine_handle::<AuthAction>().send(AuthAction::SignOut);
                                });
                            },
                            { translate!(i18, "messages.sign_out") }
                        }
                    }
                }
            }
        }
    }
}