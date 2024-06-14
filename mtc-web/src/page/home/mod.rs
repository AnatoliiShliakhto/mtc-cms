use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::router::Route::DashboardPage;

#[component]
pub fn HomePage() -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "flex flex-col p-10 gap-10 justify-center items-center grow",
            p { class: "text-4xl",
                "Military training center content management system!"
            }
            Link { class: "btn btn-neutral btn-outline w-fit",
                to: DashboardPage {},
                { translate!(i18, "messages.dashboard") }
            }
        }
    }
}