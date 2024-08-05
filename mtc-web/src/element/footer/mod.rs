use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

#[allow(dead_code)]
pub fn Footer() -> Element {
    let i18 = use_i18();

    rsx! {
        footer { class: "footer footer-center h-14 p-4 text-base-content rounded",
            a { class: "link link-hover hover:text-primary",
                href: "https://github.com/AnatoliiShliakhto/mtc-cms",
                target: "_blank",
                { translate!(i18, "messages.copyright") }
            }
        }
    }
}