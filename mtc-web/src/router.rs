#![allow(unused_variables)]

use dioxus::prelude::*;

use crate::page::administrator::administrator_page::AdministratorPage;
use crate::page::dashboard::dashboard_page::DashboardPage;
use crate::page::home::home_page::HomePage;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    #[redirect("/:..segments", | segments: Vec < String > | Route::HomePage {})]
    HomePage {},
    #[route("/administrator")]
    AdministratorPage {},
    #[route("/dashboard")]
    DashboardPage {},
}