#![allow(unused_variables)]

use dioxus::prelude::*;

use crate::component::modal_box::ModalBoxComponent;
use crate::element::footer::Footer;
use crate::element::header::Header;
use crate::page::administrator::AdministratorPage;
use crate::page::dashboard::DashboardPage;
use crate::page::home::HomePage;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
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
        div { class: "flex flex-col grow overflow",
            Outlet::<Route> {}
        }
        Footer {}
        ModalBoxComponent {}
    }
}