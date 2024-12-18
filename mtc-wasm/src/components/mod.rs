use crate::prelude::*;

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
mod form_fields_field;
mod form_permissions_field;
mod form_schema_kind_field;
mod view_html_field;
mod view_string_field;
mod view_text_field;
mod view_course_field;
mod view_links_field;
mod content_actions;
mod form_html_field;
mod storage_actions;
mod storage_box;
mod published_action;
mod form_text_area_field;
mod content_list_actions;
mod init_box;
mod search_box;
mod dialog_box;
mod personnel_actions;
mod personnel_columns_chooser;
mod form_checkbox_field;
mod view_links_search;

pub mod prelude {
    pub use super::{
        access_forbidden::*,
        breadcrumbs::*,
        menu_item::*,
        profile_controller::*,
        something_wrong::*,
        theme_switcher::*,
        loading::*,
        entries_actions::*,
        content_actions::*,
        content_list_actions::*,
        storage_actions::*,
        published_action::*,
        form_text_field::*,
        form_num_field::*,
        form_toggle_field::*,
        form_entries_field::*,
        form_select_field::*,
        form_fields_field::*,
        form_schema_kind_field::*,
        form_permissions_field::*,
        form_html_field::*,
        form_text_area_field::*,
        form_checkbox_field::*,
        entry_info_box::*,
        editor_actions::*,
        user_block_action::*,
        personnel_actions::*,
        view_html_field::*,
        view_string_field::*,
        view_text_field::*,
        view_course_field::*,
        view_links_field::*,
        view_links_search::*,
        storage_box::*,
        init_box::*,
        search_box::*,
        dialog_box::*,
        personnel_columns_chooser::*,
    };
} 