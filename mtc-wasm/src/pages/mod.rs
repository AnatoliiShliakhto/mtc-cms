use crate::prelude::*;

mod not_found;
mod sign_in;
mod change_password;
mod administrator;
mod permissions;
mod quizzes;
mod quiz_edit;
mod quiz_assign;
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
        administrator::*,
        app_data::*,
        change_password::*,
        content_edit::*,
        content_list::*,
        content_view::*,
        course_edit::*,
        group_edit::*,
        groups::*,
        js_exec::*,
        linking_qr_code::*,
        not_found::*,
        permission_create::*,
        permissions::*,
        personnel::*,
        personnel_add::*,
        qr_sign_in::*,
        quizzes::*,
        quiz_edit::*,
        quiz_assign::*,
        role_edit::*,
        roles::*,
        schema_edit::*,
        schemas::*,
        search::*,
        sign_in::*,
        user_edit::*,
        users::*,
    };
}
