use dioxus::prelude::*;
use crate::component::footer::Footer;

use crate::component::header::Header;
use crate::router::Route::DashboardPage;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Header {}
        div { class: "flex flex-col content-center w-full p-20 gap-10 content-center",
            p { class: "text-xl self-center",
                "Military training center content management system!"
            }
            Link { class: "btn btn-active btn-secondary w-fit self-center", to: DashboardPage {}, "SignIn" }
        }
        Footer {}
    }
}