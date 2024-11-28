use super::*;

#[derive(Default, Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct FileEntry {
    pub path: Cow<'static, str>,
    pub size: i32,
}