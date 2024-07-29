use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::record_model::RecordModel;

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct StringListModel {
    pub list: Vec<String>,
}

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct RecordListModel {
    pub list: Vec<RecordModel>,
}
