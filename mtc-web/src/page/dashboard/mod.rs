use dioxus::prelude::*;

use crate::element::user_dashboard::UserDashboard;

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        div { class: "flex grow items-center justify-center desktop-body-scroll",
            div { class: "flex flex-col gap-3 rounded border p-5 shadow-md min-w-96 input-bordered",
                UserDashboard {}
            }
        }
    }
}