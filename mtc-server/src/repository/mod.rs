use crate::prelude::*;

mod users;
mod permissions;
mod roles;
mod groups;
mod search_idx;
mod schemas;
mod storage_utils;
mod content;
mod system;
mod gate_passes;
mod base_repository;

pub(crate) mod prelude {
    pub(crate) use super::{
        Repository,
        groups::*,
        permissions::*,
        roles::*,
        users::*,
        schemas::*,
        search_idx::*,
        storage_utils::*,
        content::*,
        system::*,
        gate_passes::*,
        base_repository::*,
    };
}

pub(crate) struct Repository {
    database: Arc<Database>,
    config: Arc<Config>,
}

impl Repository {
    pub fn init(db: &Arc<Database>, cfg: &Arc<Config>) -> Self {
        Self {
            database: db.clone(),
            config: cfg.clone(),
        }
    }
}