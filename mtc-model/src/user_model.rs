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
}

#[derive(Deserialize, Validate)]
pub struct UserCreateModel {
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UserUpdateModel {
    pub fields: Option<Value>,
}

#[derive(Deserialize, Validate)]
pub struct UserChangePasswordModel {
    #[validate(length(min = 6, message = "Password must be 6 characters at least"))]
    pub password: String,
    #[validate(must_match(other = "password"))]
    pub confirm_password: String,
}

#[derive(Deserialize, Validate)]
pub struct UserAssignRolesModel {
    pub roles: Vec<String>,
}

#[derive(Deserialize, Validate)]
pub struct UserAssignGroupsModel {
    pub groups: Vec<String>,
}