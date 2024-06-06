use serde::{Deserialize, Deserializer, Serialize};
use surrealdb_sql::Thing;

pub mod group_model;
pub mod api_model;
pub mod auth_model;
pub mod field_model;
pub mod pagination_model;
pub mod permission_model;
pub mod role_model;
pub mod schema_model;
pub mod user_model;

pub fn from_thing<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Helper {
        Short(String),
        #[serde(with = "Thing")]
        Full(Thing),
    }

    match Helper::deserialize(deserializer)? {
        Helper::Short(value) => Ok(value),
        Helper::Full(value) => Ok(value.id.to_string())
    }
}

#[derive(Deserialize, Serialize)]
pub struct HealthModel {
    pub id: String,
}