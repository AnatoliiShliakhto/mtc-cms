use crate::prelude::*;

mod not_found;
mod home;
mod sign_in;
mod change_password;
mod administrator;
mod permissions;
mod permission_create;
mod search_result;
mod groups;
mod group_edit;
mod roles;
mod role_edit;
mod users;
mod user_edit;

pub mod prelude {
    pub use super::{
        not_found::*,
        home::*,
        sign_in::*,
        change_password::*,
        administrator::*,
        permissions::*,
        permission_create::*,
        groups::*,
        group_edit::*,
        search_result::*,
        roles::*,
        role_edit::*,
        users::*,
        user_edit::*,
    };
}
