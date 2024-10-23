use crate::prelude::*;

mod api_client;
mod auth_state;
mod menu_state;
mod i18n;
mod breadcrumbs;
mod search_engine;
mod pages_entries;
mod dialog_box;
mod personnel;
mod personnel_columns;
mod app_state;

pub mod prelude {
    pub use super::{
        app_state::*,
        api_client::*,
        auth_state::*,
        menu_state::*,
        i18n::*,
        breadcrumbs::*,
        search_engine::*,
        pages_entries::*,
        dialog_box::*,
        personnel::*,
        personnel_columns::*,
    };
}