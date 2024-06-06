use dioxus::prelude::*;
use crate::router::Route::DashboardPage;

pub fn Footer() -> Element {
    rsx! {
        footer { class: "footer footer-center p-10 bg-base-200 text-base-content rounded",
            nav { class: "grid grid-flow-col gap-4",
                Link { to: DashboardPage {}, "Dashboard" }
            }
            aside {
                p {
                    "Copyright © 2024 - All right reserved by Anatolii Shliakhto"
                }
            }
        }
    }
}