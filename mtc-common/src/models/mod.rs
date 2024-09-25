use crate::prelude::*;

mod auth_state;
mod user;
mod permission;
mod role;
mod message_kind;
mod link_entry;
mod field_kind;
mod field;
mod group;
mod entry;
mod schema_kind;
mod schema;
mod search_kind;
mod search_idx;
mod storage_entry;
mod system_info;
mod user_details;
mod user_state;
mod content;
mod course_entry;
mod search_idx_dto;

pub mod prelude {
    pub use super::{
        auth_state::*,
        user::*,
        permission::*,
        role::*,
        group::*,
        message_kind::*,
        link_entry::*,
        field_kind::*,
        field::*,
        entry::*,
        schema_kind::*,
        schema::*,
        search_kind::*,
        search_idx::*,
        storage_entry::*,
        system_info::*,
        user_details::*,
        user_state::*,
        content::*,
        course_entry::*,
        search_idx_dto::*,
    };
}
