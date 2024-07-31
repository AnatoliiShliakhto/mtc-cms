use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;

use crate::router::Route;
use crate::APP_STATE;

#[derive(PartialEq, Props, Clone)]
pub struct MainMenuItemProps {
    pub route: Route,
    pub title: String,
    pub rights: Option<String>,
    pub toggle: Signal<bool>,
}

#[component]
pub fn MainMenuItem(mut props: MainMenuItemProps) -> Element {
    let app_state = APP_STATE.peek();
    let auth_state = app_state.auth.read();

    match props.rights {
        Some(rights) => rsx! {
            if auth_state.is_permission(&rights) {
                li {
                    Link { to: props.route,
                        onclick: move |_| props.toggle.set(false),
                        { props.title }
                    }
                }
            }
        },
        None => rsx! {
            li {
                Link { to: props.route,
                    onclick: move |_| props.toggle.set(false),
                    { props.title }
                }
            }
        },
    }
}
