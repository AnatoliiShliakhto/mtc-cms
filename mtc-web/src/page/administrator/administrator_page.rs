use dioxus::prelude::*;

use crate::element::footer::Footer;
use crate::element::header::Header;

#[component]
pub fn AdministratorPage() -> Element {
    rsx! {
        Header {}
        {"Admin panel".to_string()}
        Footer {}
    }
}