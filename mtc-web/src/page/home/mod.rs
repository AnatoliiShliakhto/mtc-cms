use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::router::Route::DashboardPage;

#[component]
pub fn HomePage() -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "flex grow flex-col items-center justify-center gap-10 p-10",
            p { class: "text-4xl",
                "Military training center content management system!"
            }
            Link { class: "w-fit btn btn-neutral btn-outline",
                to: DashboardPage {},
                { translate!(i18, "messages.dashboard") }
            }
        }
    }
}