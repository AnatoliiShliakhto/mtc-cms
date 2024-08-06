use std::collections::BTreeSet;

use axum::async_trait;

use crate::error::Result;
use crate::service::system_service::SystemService;

#[async_trait]
pub trait SystemRepositoryTrait {
    async fn get_migrations(&self) -> Result<BTreeSet<String>>;
    async fn set_migrations(&self, migrations: BTreeSet<String>) -> Result<()>;
}

#[async_trait]
impl SystemRepositoryTrait for SystemService {
    async fn get_migrations(&self) -> Result<BTreeSet<String>> {
        Ok(self
            .db
            .query(r#"SELECT VALUE c_value from mtc_system WHERE c_key = 'migrations';"#)
            .await?
            .take::<Option<BTreeSet<String>>>(0)?.unwrap_or_default())
    }

    async fn set_migrations(&self, migrations: BTreeSet<String>) -> Result<()> {
        self.db
            .query(
                r#"
                UPDATE mtc_system MERGE {
	                c_value: $value,
                } WHERE c_key = 'migrations';
                "#,
            )
            .bind(("value", migrations.into_iter().collect::<Vec<String>>()))
            .await?;

        Ok(())
    }
}