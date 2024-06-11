use dioxus::prelude::*;
use dioxus_std::i18n::use_i18;
use dioxus_std::translate;

use crate::router::Route::DashboardPage;

#[component]
pub fn HomePage() -> Element {
    let i18 = use_i18();

    rsx! {
        div { class: "flex flex-col content-center w-full p-10 gap-10",
            p { class: "text-4xl self-center",
                "Military training center content management system!"
            }
            Link { class: "btn btn-neutral btn-outline w-fit self-center",
                to: DashboardPage {},
                { translate!(i18, "messages.dashboard") }
            }
        }
    }
}