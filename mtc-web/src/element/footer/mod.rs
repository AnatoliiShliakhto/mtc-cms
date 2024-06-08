use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::router::Route::DashboardPage;

pub fn Footer() -> Element {
    let i18 = use_i18();

    rsx! {
        footer { class: "footer footer-center p-4 min-w-72 bg-base-200 text-base-content rounded",
            nav { class: "grid grid-flow-col gap-4",
                Link { to: DashboardPage {}, { translate!(i18, "messages.dashboard") } }
            }
            aside {
                p { class: "flex flex-col gap-1",
                    span { class: "flex flex-row gap-1",
                        { translate!(i18, "messages.copyright") }
                        a { class: "link link-hover",
                            href: "mailto:a.shlyakhto@gmail.com",
                            "<a.shlyakhto@gmail.com>"
                        }
                    }
                    a { class: "link",
                        href: "https://github.com/AnatoliiShliakhto/mtc-cms",
                        { translate!(i18, "messages.project_github") }
                    }
                }
            }
        }
    }
}