use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::APP_STATE;
use crate::router::Route::HomePage;

pub fn Breadcrumbs() -> Element {
    let app_state = APP_STATE.peek();
    let items = app_state.breadcrumbs.signal();
    let i18 = use_i18();

    rsx! {
        div { class: "breadcrumbs text-sm",
            ul {
                li {
                    a { class: "gap-2",
                        onclick: move |_| { navigator().push(HomePage {}); },
                        Icon { 
                            width: 16,
                            height: 16,
                            fill: "currentColor",
                            icon: dioxus_free_icons::icons::md_action_icons::MdHome
                        }
                        { translate!(i18, "messages.home") }
                    }
                }
                for item in items() {
                    if item.slug.is_empty() {
                        li {
                            span { class: "inline-flex items-center gap-2",
                                { item.title.clone() } 
                            }
                        }
                    } else {
                        li {
                            a {
                                onclick: move |_| { navigator().push(item.slug.clone()); },
                                { item.title.clone() }
                            }
                        }
                    }                    
                }
            }            
        }
    }
}