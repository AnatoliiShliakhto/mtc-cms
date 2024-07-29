use serde::{Deserialize, Serialize};
use serde_json::Value;
use surrealdb_sql::Datetime;
use validator::Validate;

use crate::from_thing;

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct UserModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub login: String,
    #[serde(skip_serializing, default)]
    pub password: String,
    pub blocked: bool,
    pub access_level: i32,
    pub access_count: i32,
    pub last_access: Option<Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Value>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

impl Default for UserModel {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            login: "".to_string(),
            password: "".to_string(),
            blocked: false,
            access_level: 999,
            access_count: 0,
            last_access: None,
            fields: None,
            created_at: Default::default(),
            updated_at: Default::default(),
            created_by: "".to_string(),
            updated_by: "".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserCreateModel {
    pub blocked: bool,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserUpdateModel {
    pub blocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Value>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserChangePasswordModel {
    #[validate(length(min = 6, message = "Password must be 6 characters at least"))]
    pub old_password: String,
    #[validate(length(min = 6, message = "Password must be 6 characters at least"))]
    pub new_password: String,
}
