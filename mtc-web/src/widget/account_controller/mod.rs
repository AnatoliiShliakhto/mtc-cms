use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_regular_icons::FaUser;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use mtc_model::auth_model::AuthModelTrait;

use crate::action::auth_action::AuthAction;
use crate::global_signal::APP_AUTH;
use crate::router::Route::DashboardPage;

#[component]
pub fn AccountControllerWidget() -> Element {
    let i18 = use_i18();
    let auth = APP_AUTH.read().is_auth();

    let account_widget_color = match auth {
        true => "green".to_string(),
        false => "currentColor".to_string(),
    };

    rsx! {
        if !auth {
            Link { class: "btn btn-ghost join-item",
                to: DashboardPage {},
                Icon {
                    width: 20,
                    height: 20,
                    fill: account_widget_color,
                    icon: FaUser
                }
            }
        } else {
            div { class: "dropdown dropdown-end join-item",
                div { tabindex: "0", role: "button", class: "btn btn-ghost join-item",
                    Icon {
                        width: 20,
                        height: 20,
                        fill: account_widget_color,
                        icon: FaUser
                    }
                }
                ul { tabindex: "0", class: "dropdown-content z-[1] menu p-2 shadow-md bg-base-100 rounded-md w-52 mt-2",
                    li { Link { to: DashboardPage {}, { translate!(i18, "messages.dashboard") } } }
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