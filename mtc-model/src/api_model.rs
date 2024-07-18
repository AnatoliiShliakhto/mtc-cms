use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb_sql::Datetime;
use validator::Validate;

use crate::from_thing;

#[derive(Serialize, Deserialize)]
pub struct ApiModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub title: String,
    pub fields: Option<Value>,
    pub published: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

impl Default for ApiModel {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            slug: "".to_string(),
            title: "".to_string(),
            fields: None,
            published: false,
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: "".to_string(),
            updated_by: "".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ApiPostModel {
    pub title: String,
    pub published: bool,
    pub fields: Option<Value>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ApiListItemModel{
    pub slug: String,
    pub title: String,
    pub published: bool,
}
