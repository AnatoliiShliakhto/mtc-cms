use serde::{Deserialize, Serialize};
use surrealdb_sql::Datetime;
use validator::Validate;

use crate::from_thing;

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct RoleModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub slug: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,    
}

impl Default for RoleModel {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            slug: "".to_string(),
            title: "".to_string(),
            permissions: None,
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: "".to_string(),
            updated_by: "".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Validate, Clone)]
pub struct RoleCreateModel {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RoleUpdateModel {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

#[derive(Default, Deserialize, Serialize, Validate)]
pub struct RolesModel {
    pub roles: Vec<String>,
}