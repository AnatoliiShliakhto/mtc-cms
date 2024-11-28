use crate::prelude::*;

mod not_found;
mod sign_in;
mod change_password;
mod administrator;
mod permissions;
mod permission_create;
mod search;
mod groups;
mod group_edit;
mod roles;
mod role_edit;
mod users;
mod user_edit;
mod schemas;
mod schema_edit;
mod content_list;
mod content_view;
mod content_edit;
mod course_edit;
mod personnel;
mod personnel_add;
mod linking_qr_code;
mod qr_sign_in;
mod js_exec;
mod app_data;

pub mod prelude {
    pub use super::{
        not_found::*,
        sign_in::*,
        change_password::*,
        administrator::*,
        permissions::*,
        permission_create::*,
        groups::*,
        group_edit::*,
        search::*,
        roles::*,
        role_edit::*,
        users::*,
        user_edit::*,
        schemas::*,
        schema_edit::*,
        content_list::*,
        content_view::*,
        content_edit::*,
        course_edit::*,
        personnel::*,
        personnel_add::*,
        linking_qr_code::*,
        qr_sign_in::*,
        js_exec::*,
        app_data::*,
    };
}
