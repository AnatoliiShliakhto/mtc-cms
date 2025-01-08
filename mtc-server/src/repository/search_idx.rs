use super::*;

pub trait SearchIdxRepository {
    fn find_search_idx(
        &self,
        permissions: BTreeSet<Cow<'static, str>>
    ) -> impl Future<Output = Result<Vec<SearchIdxDto>>> + Send;
    fn search_content(
        &self,
        search_string: Cow<'static, str>,
        permissions: BTreeSet<Cow<'static, str>>,
    ) -> impl Future<Output = Result<Vec<SearchIdxDto>>> + Send;
}

impl SearchIdxRepository for Repository {
    /// Finds all search index records that the current user has access to.
    ///
    /// The current user must have at least one of the provided permissions in
    /// order to access the record.
    ///
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

    /// Searches for content within the search index that matches the specified search string.
    ///
    /// The search results are filtered based on the permissions provided, ensuring that
    /// only records accessible to the current user are returned. The results are ordered
    /// by their search score in descending order, and a maximum of 20 results are returned.
    ///
    /// # Arguments
    ///
    /// * `search_string` - The string to search for within the titles of the search index.
    /// * `permissions` - A set of permissions that the current user has, used to filter
    ///   the search results.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of [`SearchIdxDto`] records that match the search
    /// criteria, or an error if the query fails.
    async fn search_content(
        &self,
        search_string: Cow<'static, str>,
        permissions: BTreeSet<Cow<'static, str>>,
    ) -> Result<Vec<SearchIdxDto>> {
        let sql = r#"
            SELECT *, search::score(1) AS score FROM search_index
            WHERE title @1@ $search_string AND permission in $permissions
            ORDER BY score DESC LIMIT 20;
        "#;

        let search_idx = self
            .database
            .query(sql)
            .bind(("search_string", search_string))
            .bind(("permissions", permissions))
            .await?
            .take::<Vec<SearchIdxDto>>(0)?;

        Ok(search_idx)
    }
}