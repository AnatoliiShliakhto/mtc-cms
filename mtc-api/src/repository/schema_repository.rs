use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::schema_model::{SchemaCreateModel, SchemaModel};
use crate::repository::RepositoryPaginate;
use crate::repository_paginate;
use crate::service::schema_service::SchemaService;

repository_paginate!(SchemaService, SchemaModel, "schemas");

#[async_trait]
pub trait SchemaRepositoryTrait {
    async fn find(&self, id: &str) -> Result<SchemaModel>;
    async fn find_by_name(&self, name: &str) -> Result<SchemaModel>;
    async fn can_create(&self, name: &str) -> Result<bool>;
    async fn create(&self, model: SchemaCreateModel) -> Result<SchemaModel>;
    async fn delete(&self, name: &str) -> Result<()>;
}

#[async_trait]
impl SchemaRepositoryTrait for SchemaService {
    async fn find(
        &self,
        id: &str,
    ) -> Result<SchemaModel> {
        let result: Option<SchemaModel> = self.db.query(r#"
            SELECT * FROM type::thing('schemas', $id);
            "#)
            .bind(("id", id.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn find_by_name(
        &self,
        name: &str,
    ) -> Result<SchemaModel> {
        let result: Option<SchemaModel> = self.db.query(r#"
            SELECT * FROM schemas WHERE name=$name;
            "#)
            .bind(("name", name.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn can_create(
        &self,
        name: &str,
    ) -> Result<bool> {
        let result: Option<String> = self.db.query(r#"
            SELECT name FROM schemas WHERE name=$name;
            "#)
            .bind(("name", name))
            .await?
            .take(0)?;

        match result {
            Some(..) => Ok(false),
            None => Ok(true)
        }
    }

    async fn create(
        &self,
        model: SchemaCreateModel,
    ) -> Result<SchemaModel> {
        if !self.can_create(&model.name).await? {
            Err(ApiError::from(DbError::EntryAlreadyExists))?
        }

        let result: Option<SchemaModel> = self.db.query(r#"
            BEGIN TRANSACTION;

            CREATE schemas CONTENT {
	            name: $name,
	            is_collection: $is_collection
            };

            CREATE permissions CONTENT {
                id: $permission_read_id,
                name: $permission_read_name
            };

            CREATE permissions CONTENT {
                id: $permission_write_id,
                name: $permission_write_name
            };

            CREATE permissions CONTENT {
                id: $permission_delete_id,
                name: $permission_delete_name
            };

            COMMIT TRANSACTION;
            "#)
            .bind(("name", &model.name))
            .bind(("is_collection", &model.is_collection))
            .bind(("permission_read_id", format!("{}_read", &model.name)))
            .bind(("permission_read_name", format!("{}::read", &model.name)))
            .bind(("permission_write_id", format!("{}_write", &model.name)))
            .bind(("permission_write_name", format!("{}::write", &model.name)))
            .bind(("permission_delete_id", format!("{}_delete", &model.name)))
            .bind(("permission_delete_name", format!("{}::delete", &model.name)))
            .await?
            .take(0)?;

        match result {
            Some(value) => {
                if model.is_collection {
                    self.db.query(format!(r#"
                    BEGIN TRANSACTION;
                    DEFINE TABLE {0};
                    DEFINE FIELD fields ON TABLE {0} TYPE option<array>;
                    DEFINE FIELD created_at ON TABLE {0} TYPE datetime DEFAULT time::now();
                    DEFINE FIELD updated_at ON TABLE {0} TYPE datetime VALUE time::now();
                    COMMIT TRANSACTION;
                    "#, model.name)).await?;
                }
                Ok(value)
            }
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }

    async fn delete(&self, name: &str) -> Result<()> {
        let model = self.find_by_name(name).await?;

        if model.is_system {
            Err(ApiError::from(DbError::EntryDelete))?
        }

        self.db.query(r#"
            BEGIN TRANSACTION;

            DELETE FROM schemas WHERE name=$name;

            DELETE type::thing('permissions', $permission_read_id);
            DELETE type::thing('permissions', $permission_write_id);
            DELETE type::thing('permissions', $permission_delete_id);

            COMMIT TRANSACTION;
            "#)
            .bind(("name", &model.name))
            .bind(("permission_read_id", format!("{}_read", &model.name)))
            .bind(("permission_write_id", format!("{}_write", &model.name)))
            .bind(("permission_delete_id", format!("{}_delete", &model.name)))
            .await?;

        if model.is_collection {
            self.db.query(format!(r#"
                REMOVE TABLE IF EXISTS {};
                "#, &model.name))
                .await?;
        }

        Ok(())
    }
}
