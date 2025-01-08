use super::*;

pub trait GroupsRepository {
    fn find_group_list(&self)
        -> impl Future<Output = Result<Vec<Entry>>> + Send;
    fn find_group_by_login(&self, login: Cow<'static, str>)
        -> impl Future<Output = Result<Cow<'static, str>>> + Send;
    fn find_group(&self, id: Cow<'static, str>)
        -> impl Future<Output = Result<Group>> + Send;
    fn update_group(&self, payload: Value, by: Cow<'static, str>)
        -> impl Future<Output = Result<()>> + Send;
    fn delete_group(&self, id: Cow<'static, str>)
        -> impl Future<Output = Result<()>> + Send;
    fn assign_group_to_user(
        &self,
        id: Cow<'static, str>,
        group: Cow<'static, str>,
    ) -> impl Future<Output = Result<()>> + Send;
}

impl GroupsRepository for Repository {
    /// Finds a list of all groups.
    ///
    /// # Response
    ///
    /// A JSON response with a list of groups as [`Entry`].
    async fn find_group_list(&self) -> Result<Vec<Entry>> {
        let sql = r#"
        SELECT record::id(id) as id, slug, title FROM groups ORDER BY slug;
        "#;

        let groups = self
            .database
            .query(sql)
            .await?
            .take::<Vec<Entry>>(0)?;

        Ok(groups)
    }

    /// Finds the group of a user by login.
    ///
    /// # Arguments
    ///
    /// * `login`: The login of the user.
    ///
    /// # Response
    ///
    /// A JSON response with the slug of the group as a string.
    /// If the user does not exist, an empty string is returned.
    async fn find_group_by_login(&self, login: Cow<'static, str>) -> Result<Cow<'static, str>> {
        let sql = r#"
        array::at(SELECT VALUE ->user_groups->groups.slug FROM users WHERE login=$login, 0);
        "#;

        let group = self.database.query(sql)
            .bind(("login", login))
            .await?
            .take::<Option<Cow<str>>>(0)?
            .unwrap_or("".into());

        Ok(group)
    }

    /// Finds a group by ID.
    ///
    /// # Arguments
    ///
    /// * `id`: The ID of the group to find.
    ///
    /// # Response
    ///
    /// A JSON response with the found group as a [`Group`].
    /// If the group does not exist, an error is returned.
    async fn find_group(&self, id: Cow<'static, str>) -> Result<Group> {
        let sql = r#"
        SELECT *, record::id(id) as id FROM ONLY type::record("groups:" + $id);
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id))
            .await?
            .take::<Option<Group>>(0)?
            .ok_or(DatabaseError::EntryNotFound.into())
    }

    /// Updates a group by ID.
    ///
    /// # Arguments
    ///
    /// * `payload`: A JSON payload with the new values for the group.
    /// * `by`: The login of the user making the change.
    ///
    /// # Response
    ///
    /// * Ok(()) if the group was updated successfully.
    ///
    /// If the group does not exist and the `id` field is not present in the payload,
    /// a new group is created with the provided values.
    /// If the group does not exist and the `id` field is present in the payload,
    /// an error is returned.
    async fn update_group(&self, payload: Value, by: Cow<'static, str>) -> Result<()> {
        let mut sql = vec![];
        let id = payload.key_str("id").unwrap_or_default();
        let slug = payload.key_str("slug").unwrap_or_default();
        let title = payload.key_str("title").unwrap_or_default();

        if payload.contains_key("id") && !id.is_empty() {
            sql.push(r#"UPDATE type::record("groups:" + $id) MERGE {"#)
        } else {
            sql.push(r#"
            CREATE groups CONTENT {
                created_by: $by,
            "#)
        }

        if payload.contains_key("slug") {
            sql.push(r#"
            slug: $slug,
            "#)
        }

        if payload.contains_key("title") {
            sql.push(r#"
            title: $title,
            "#)
        }

        sql.push(r#"
            updated_by: $by
        };
        "#);

        self
            .database
            .query(sql.concat())
            .bind(("id", id))
            .bind(("slug", slug))
            .bind(("title", title))
            .bind(("by", by))
            .await?;

        Ok(())
    }

    /// Deletes a group by ID.
    ///
    /// # Arguments
    ///
    /// * `id`: The ID of the group to delete.
    ///
    /// # Response
    ///
    /// * Ok(()) if the group was deleted successfully.
    ///
    /// If the group does not exist, an error is returned.
    async fn delete_group(&self, id: Cow<'static, str>) -> Result<()> {
        let sql = r#"
            DELETE type::record("groups:" + $id);
        "#;

        self
            .database
            .query(sql)
            .bind(("id", id))
            .await?;

        Ok(())
    }


    /// Assigns a group to a user.
    ///
    /// # Arguments
    ///
    /// * `id`: The ID of the user to assign the group to.
    /// * `group`: The ID of the group to assign.
    ///
    /// # Response
    ///
    /// * `Ok(())` if the group was assigned successfully.
    ///
    /// If the group does not exist, an error is returned.
    async fn assign_group_to_user(&self, id: Cow<'static, str>, group: Cow<'static, str>) -> Result<()> {
        let mut sql = vec!["BEGIN TRANSACTION;"];
        let drop_groups = format!(r#"
            DELETE users:{}->user_groups;
        "#, id);
        sql.push(&drop_groups);

            let group_query = format!(r#"
                RELATE users:{}->user_groups->groups:{};
            "#, id, group);
        if !group.is_empty() {
            sql.push(&group_query);
        }
        sql.push("COMMIT TRANSACTION;");

        self
            .database
            .query(sql.concat())
            .await?;

        Ok(())
    }
}