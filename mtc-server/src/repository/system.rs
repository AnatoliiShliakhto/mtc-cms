use super::*;

#[async_trait]
pub trait SystemTrait {
    async fn find_migrations(&self) -> Result<BTreeSet<Cow<'static, str>>>;
    async fn update_migrations(&self, migrations: BTreeSet<Cow<'static, str>>) -> Result<()>;
    async fn migrate(
        &self,
        sql: Cow<'static, str>,
        user: Cow<'static, str>,
        password: Cow<'static, str>,
    ) -> Result<()>;

    async fn find_system_info(&self) -> Result<SystemInfo>;
    async fn rebuild_index(&self) -> Result<()>;
}

#[async_trait]
impl SystemTrait for Repository {
    async fn find_migrations(&self) -> Result<BTreeSet<Cow<'static, str>>> {
        Ok(self
            .database
            .query(r#"SELECT VALUE c_value from mtc_system WHERE c_key = 'migrations';"#)
            .await?
            .take::<Option<BTreeSet<Cow<'static, str>>>>(0)?.unwrap_or_default())
    }

    async fn update_migrations(&self, migrations: BTreeSet<Cow<'static, str>>) -> Result<()> {
        self.database
            .query(
                r#"
                UPDATE mtc_system MERGE {
	                c_value: $value,
                } WHERE c_key = 'migrations';
                "#,
            )
            .bind(("value", migrations.into_iter().collect::<Vec<Cow<'static, str>>>()))
            .await?;

        Ok(())
    }

    async fn migrate(
        &self,
        sql: Cow<'static, str>,
        user: Cow<'static, str>,
        password: Cow<'static, str>
    ) -> Result<()> {
        self.database
            .query(&*sql)
            .bind(("login", user))
            .bind(("password", password))
            .await?;

        Ok(())
    }

    async fn find_system_info(&self) -> Result<SystemInfo> {
        Ok(self
            .database
            .query(r#"SELECT VALUE c_value from mtc_system WHERE c_key = 'info';"#)
            .await?
            .take::<Option<SystemInfo>>(0)?.unwrap_or_default())
    }

    async fn rebuild_index(&self) -> Result<()> {
        let mut sql: Vec<&str> = vec![];


        self.database.query(sql.concat()).await?;

        Ok(())
    }
}