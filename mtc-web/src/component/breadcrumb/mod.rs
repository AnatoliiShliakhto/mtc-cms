use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use crate::router::Route::HomePage;

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    pub title: String,
}

//todo Breadcrumb
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
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
            span { class: "inline-flex flex-nowrap items-center",
                Icon {
                    width: 22,
                    height: 22,
                    fill: "currentColor",
                    icon: dioxus_free_icons::icons::md_navigation_icons::MdChevronRight
                }
                button { class: "btn btn-ghost btn-sm px-1", 
                    { props.title }
                }
            }
        }
    }
}
