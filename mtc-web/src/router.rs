#![allow(unused_variables)]

use dioxus::prelude::*;

use crate::page::administrator::administrator_page::AdministratorPage;
use crate::page::home_page::HomePage;
use crate::page::dashboard_page::DashboardPage;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    #[redirect("/:..segments", |segments: Vec<String>| Route::HomePage {})]
    HomePage {},
    #[route("/administrator")]
    AdministratorPage {},
    #[route("/dashboard")]
    DashboardPage {},
}