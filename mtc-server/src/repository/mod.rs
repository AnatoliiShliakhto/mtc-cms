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