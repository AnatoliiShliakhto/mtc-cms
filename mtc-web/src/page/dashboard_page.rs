use dioxus::prelude::*;

use crate::component::footer::Footer;
use crate::component::header::Header;
use crate::component::user_dashboard::UserDashboard;

#[component]
pub fn DashboardPage() -> Element {
    rsx! {
        Header {}
        UserDashboard {}
        Footer {}
    }
}