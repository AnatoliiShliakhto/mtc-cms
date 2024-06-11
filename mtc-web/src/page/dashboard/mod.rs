use dioxus::prelude::*;

use crate::element::user_dashboard::UserDashboard;

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        UserDashboard {}
    }
}