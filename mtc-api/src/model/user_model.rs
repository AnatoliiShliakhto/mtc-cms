use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use validator::Validate;
use crate::model::field_model::FieldModel;

use crate::model::from_thing;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UserModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub login: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub blocked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<FieldModel>>,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Deserialize, Validate)]
pub struct UserCreateModel {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UserUpdateModel {
    pub fields: Option<Vec<FieldModel>>,
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