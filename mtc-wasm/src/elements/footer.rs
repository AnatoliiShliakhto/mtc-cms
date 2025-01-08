use super::*;

/// A footer component designed to be centered with specific styling.
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { 
            class: "footer footer-center h-14 p-4 text-base-content rounded",
            a {
                class: "link link-hover hover:text-primary",
                href: "https://github.com/AnatoliiShliakhto/mtc-cms",
                "onclick": "linkOpen(this); return false;",
                { t!("site-copyright") }
            }
        }
    }
}