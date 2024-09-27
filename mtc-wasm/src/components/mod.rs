use crate::prelude::*;

mod message_box;
mod profile_controller;
mod theme_switcher;
mod menu_item;
mod something_wrong;
mod breadcrumbs;
mod access_forbidden;
pub mod loading;
mod entries_actions;
mod form_text_field;
mod entry_info_box;
mod editor_actions;
mod form_num_field;
mod form_toggle_field;
mod form_entries_field;
mod form_select_field;
mod user_block_action;

pub mod prelude {
    pub use super::{
        access_forbidden::*,
        breadcrumbs::*,
        menu_item::*,
        message_box::*,
        profile_controller::*,
        something_wrong::*,
        theme_switcher::*,
        loading::*,
        entries_actions::*,
        form_text_field::*,
        form_num_field::*,
        form_toggle_field::*,
        form_entries_field::*,
        form_select_field::*,
        entry_info_box::*,
        editor_actions::*,
        user_block_action::*,
    };
} 