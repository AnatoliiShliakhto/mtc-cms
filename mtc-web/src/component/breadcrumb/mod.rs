use dioxus::prelude::*;
use dioxus_free_icons::Icon;

#[derive(Props, Clone, PartialEq)]
pub struct BreadcrumbProps {
    pub title: String,
}

//todo Breadcrumb
pub fn Breadcrumb(props: BreadcrumbProps) -> Element {
    rsx! {
        div { class: "inline-flex flex-nowrap items-center",
            Icon {
                width: 22,
                height: 22,
                fill: "currentColor",
                icon: dioxus_free_icons::icons::md_action_icons::MdHome
            }
            Icon {
                width: 22,
                height: 22,
                fill: "currentColor",
                icon: dioxus_free_icons::icons::md_navigation_icons::MdChevronRight
            }
            span { class: "text-xl", { props.title } }
        }
    }
}
