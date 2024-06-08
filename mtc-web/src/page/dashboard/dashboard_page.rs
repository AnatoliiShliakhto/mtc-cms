use dioxus::prelude::*;

use crate::element::footer::Footer;
use crate::element::header::Header;
use crate::element::user_dashboard::UserDashboard;

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        Header {}
        UserDashboard {}
        Footer {}
    }
}