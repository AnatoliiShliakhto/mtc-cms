use super::*;

pub trait SearchIdxRepository {
    async fn find_search_idx(
        &self,
        permissions: BTreeSet<Cow<'static, str>>,
    ) -> Result<Vec<SearchIdxDto>>;
    async fn search_content(
        &self,
        search_string: impl ToString,
        permissions: BTreeSet<Cow<'static, str>>,
    ) -> Result<Vec<SearchIdxDto>>;
}

impl SearchIdxRepository for Repository {
    async fn find_search_idx(
        &self,
        permissions: BTreeSet<Cow<'static, str>>,
    ) -> Result<Vec<SearchIdxDto>> {
        let sql = r#"
            SELECT * OMIT id, permission FROM search_index WHERE permission in $permissions;
        "#;

        self
            .database
            .query(sql)
            .bind(("permissions", permissions))
            .await?
            .take::<Vec<SearchIdxDto>>(0)
            .map(Ok)?
    }

    async fn search_content(
        &self,
        search_string: impl ToString,
        permissions: BTreeSet<Cow<'static, str>>,
    ) -> Result<Vec<SearchIdxDto>> {
        let sql = r#"
            SELECT *, search::score(1) AS score FROM search_index
            WHERE title @1@ $search_string AND permission in $permissions
            ORDER BY score DESC LIMIT 20;
        "#;

        self
            .database
            .query(sql)
            .bind(("search_string", search_string.to_string()))
            .bind(("permissions", permissions))
            .await?
            .take::<Vec<SearchIdxDto>>(0)
            .map(Ok)?
    }
}