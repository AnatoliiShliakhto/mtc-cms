use super::*;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct FileAsset {
    pub name: Cow<'static, str>,
    pub size: usize,
}
