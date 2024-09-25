use super::*;

#[async_trait]
pub trait SearchIdxRepository {
    async fn find_search_idx(
        &self,
        permissions: BTreeSet<Cow<'static, str>>
    ) -> Result<Vec<SearchIdxDto>>;
}

#[async_trait]
impl SearchIdxRepository for Repository {
    async fn find_search_idx(
        &self,
        permissions: BTreeSet<Cow<'static, str>>,
    ) -> Result<Vec<SearchIdxDto>> {
        let sql = r#"
            SELECT * OMIT id, permission FROM search_index WHERE permission in $permissions;
        "#;

        let search_idx = self
            .database
            .query(sql)
            .bind(("permissions", permissions))
            .await?
            .take::<Vec<SearchIdxDto>>(0)?;

        Ok(search_idx)
    }
}