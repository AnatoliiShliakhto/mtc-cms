use crate::prelude::*;

mod local_storage;
mod session_storage;
mod indexed_db_storage;

pub mod prelude {
    pub use super::{
        local_storage::*,
        session_storage::*,
        indexed_db_storage::*,
    };
}

pub struct StorageEntry<T> {
    key: String,
    value: T,
}