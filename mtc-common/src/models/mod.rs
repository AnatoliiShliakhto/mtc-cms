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
mod asset;
mod system_info;
mod user_details;
mod content;
mod course_entry;
mod search_idx_dto;
mod schema_entry_details;

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
        schema_entry_details::*,
        search_kind::*,
        search_idx::*,
        asset::*,
        system_info::*,
        user_details::*,
        content::*,
        course_entry::*,
        search_idx_dto::*,
    };
}
