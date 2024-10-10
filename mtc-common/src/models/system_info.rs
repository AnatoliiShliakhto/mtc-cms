use super::*;
#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Default)]
pub struct SystemInfo {
    pub pages: i32,
    pub media: i32,
    pub files: i32,
    pub links: i32,
    pub users: i32,
    pub active_users: i32,
    pub courses: i32,
    pub indexes: i32,
}