use crate::prelude::*;

mod api_client;
mod auth_state;
mod menu_state;
mod i18n;
mod breadcrumbs;
mod search_engine;
mod pages_entries;
mod dialog_box;

pub mod prelude {
    pub use super::{
        api_client::*,
        auth_state::*,
        menu_state::*,
        i18n::*,
        breadcrumbs::*,
        search_engine::*,
        pages_entries::*,
        dialog_box::*,
    };
}