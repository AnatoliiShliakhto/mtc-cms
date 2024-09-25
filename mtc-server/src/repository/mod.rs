use crate::prelude::*;

mod users;
mod permissions;
mod roles;
mod groups;
mod search_idx;

pub mod prelude {
    pub use super::{
        Repository,
        groups::*,
        permissions::*,
        roles::*,
        users::*,
        search_idx::*,
    };
}

pub struct Repository {
    database: Arc<Database>,
}

impl Repository {
    pub fn init(db: &Arc<Database>) -> Self {
        Self {
            database: db.clone(),
        }
    }
}