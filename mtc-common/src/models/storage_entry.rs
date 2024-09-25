use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct StorageEntry {
    #[serde(borrow)]
    pub name: Cow<'static, str>,
    pub size: usize,
}
