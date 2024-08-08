use serde::{Deserialize, Serialize};
use surrealdb_sql::Datetime;

#[derive(Serialize, Default, Debug, Deserialize, Clone, PartialEq)]
pub struct UserDetailsModel {
    pub rank: String,
    pub name: String,
}

#[derive(Serialize, Default, Debug, Deserialize, Clone, PartialEq)]
pub struct UserDetailsStateModel {
    pub login: String,
    pub blocked: bool,
    pub last_access: Datetime,
    pub access_count: i32,
}
