use super::*;

/// File entry DTO
#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FileEntry {
    pub path: Cow<'static, str>,
    pub size: i32,
}