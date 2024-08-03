use dioxus::prelude::*;
use dioxus_free_icons::Icon;

use crate::APP_STATE;
use crate::router::Route::HomePage;

pub fn Breadcrumbs() -> Element {
    let app_state = APP_STATE.peek();
    let items = app_state.breadcrumbs.signal();

    rsx! {
        div { class: "flex flex-wrap items-center",
            Link { class: "btn btn-ghost btn-sm px-1",
                to: HomePage {},
                Icon {
                    width: 22,
                    height: 22,
                    fill: "currentColor",
                    icon: dioxus_free_icons::icons::md_action_icons::MdHome
                }
            }
            for item in items() {
                span { class: "inline-flex flex-nowrap items-center",
                    Icon { class: "mt-[3px]",
                        width: 22,
                        height: 22,
                        fill: "currentColor",
                        icon: dioxus_free_icons::icons::md_navigation_icons::MdChevronRight
                    }
                    if item.slug.is_empty() {
                        span { class: "inline-flex flex-nowrap h-8 min-h-8 px-1 items-center font-semibold text-sm",
                            { item.title.clone() }
                        }
                    } else {
                        button { class: "btn btn-ghost btn-sm px-1",
                            onclick: move |_| {
                                navigator().push(item.slug.clone());
                            },
                            { item.title.clone() }
                        }
                    }    
                }    
            }
        }    
    }
}