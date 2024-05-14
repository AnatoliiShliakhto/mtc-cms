use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::model::from_thing;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UserModel {
    #[serde(deserialize_with = "from_thing")]
    pub id: String,
    pub login: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub blocked: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
}

#[derive(Deserialize)]
pub struct UserCreateModel {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserUpdateModel {
    pub login: String,
}

#[derive(Deserialize)]
pub struct UserChangePasswordModel {
    pub password: String,
    pub confirm_password: String,
}