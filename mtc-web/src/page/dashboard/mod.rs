use dioxus::prelude::*;

use crate::element::user_dashboard::UserDashboard;

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        div { class: crate::DIV_CENTER,
            div { class: "flex w-full flex-col rounded border p-5 shadow-md max-w-96 input-bordered",
                UserDashboard {}
            }
        }
    }
}