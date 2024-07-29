use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone)]
pub struct StorageModel {
    pub name: String,
    pub size: usize,
}

#[derive(Default, Deserialize, Serialize, Validate)]
pub struct StoragesModel {
    pub files: Vec<StorageModel>,
}