use dioxus::prelude::*;

use mtc_model::auth_model::AuthModelTrait;

use crate::global_signal::APP_AUTH;
use crate::page::administrator::dashboard::Dashboard;
use crate::page::not_found::NotFoundPage;

mod dashboard;

#[allow(dead_code)]
pub enum AdministratorRouteModel {
    Dashboard,
    Group,
    User,
}

#[component]
pub fn AdministratorPage() -> Element {
    if APP_AUTH.read().is_permission("administrator").eq(&false) {
        return rsx! { NotFoundPage {} };
    }

    let administrator_route =
        use_context_provider(|| Signal::new(AdministratorRouteModel::Dashboard));

//    let test = &*administrator_route.read();

    rsx! {
        div { class: "flex flex-row w-full h-full divide-x divide-slate-400/25",
            div { class: "min-w-32 p-2",
                "Menu"
            }
            match &*administrator_route.read() {
                AdministratorRouteModel::Dashboard => rsx! { Dashboard {} },
                _ => rsx! { Dashboard {} },
            }
        }
    }
}