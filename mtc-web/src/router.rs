use dioxus::prelude::*;

use crate::component::modal_box::ModalBoxComponent;
use crate::element::header::Header;
use crate::page::administrator::AdministratorPage;
use crate::page::dashboard::DashboardPage;
use crate::page::home::HomePage;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(RootLayout)]
    #[route("/")]
    #[redirect("/:..segments", | segments: Vec < String > | Route::HomePage {})]
    HomePage {},
    #[route("/administrator")]
    AdministratorPage {},
    #[route("/dashboard")]
    DashboardPage {},
}

#[component]
fn RootLayout() -> Element {
    rsx! {
        Header {}
        div { class: "flex grow flex-col overflow",
            Outlet::<Route> {}
        }
//        Footer {}
        ModalBoxComponent {}
    }
}