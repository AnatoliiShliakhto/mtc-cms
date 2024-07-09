use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone)]
pub struct StoreModel {
    pub name: String,
    pub size: usize,
}

#[derive(Default, Deserialize, Serialize, Validate)]
pub struct StoresModel {
    pub files: Vec<StoreModel>,
}