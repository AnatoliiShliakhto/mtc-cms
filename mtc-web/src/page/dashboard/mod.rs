use dioxus::prelude::*;

use crate::element::user_dashboard::UserDashboard;

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        div { class: "flex justify-center items-center grow",
            div { class: "flex flex-col gap-3 p-5 min-w-96 border input-bordered shadow-md rounded",
                UserDashboard {}
            }
        }
    }
}