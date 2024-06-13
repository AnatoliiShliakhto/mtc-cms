use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

pub fn Footer() -> Element {
    let i18 = use_i18();

    rsx! {
        footer { class: "footer footer-center absolute bottom-0 left-0 right-0 h-14 p-4 min-w-72 bg-base-200 text-base-content rounded",
            aside {
                p { class: "flex flex-wrap gap-1",
                    a { class: "link link-hover",
                        href: "mailto:a.shlyakhto@gmail.com",
                        { translate!(i18, "messages.copyright") }
                    }
                    a { class: "link link-hover",
                        href: "https://github.com/AnatoliiShliakhto/mtc-cms",
                        target: "_blank",
                        "(GitHub)"
                    }
                }
            }
        }
    }
}