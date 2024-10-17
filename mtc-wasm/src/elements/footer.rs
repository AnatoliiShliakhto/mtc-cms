use super::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { 
            class: "footer footer-center h-14 p-4 text-base-content rounded",
            a { 
                class: "link link-hover hover:text-primary",
                href: "https://github.com/AnatoliiShliakhto/mtc-cms",
                target: "_blank",
                { t!("site-copyright") }
            }
        }
    }
}