use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct RecordModel {
    pub slug: String,
    pub title: String,
}    

