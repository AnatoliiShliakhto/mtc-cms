use crate::prelude::*;

mod sync;
mod auth;
mod permissions;
mod groups;
mod roles;
mod users;
mod schemas;
mod content;
mod storage;
mod system;
mod index_html;
mod gate_passes;

pub(crate) mod prelude {
    pub(crate) use super::{
        auth::*,
        sync::*,
        permissions::*,
        groups::*,
        roles::*,
        users::*,
        schemas::*,
        content::*,
        storage::*,
        system::*,
        index_html::*,
        gate_passes::*,
    };
}