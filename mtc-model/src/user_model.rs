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
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserUpdateModel {
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
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UsersModel {
    pub users: Vec<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserAssignRolesModel {
    pub roles: Vec<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserAssignGroupsModel {
    pub groups: Vec<String>,
}