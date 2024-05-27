use axum::async_trait;

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::model::schema_model::{SchemaCreateModel, SchemaFieldsModel, SchemaModel, SchemaUpdateModel};
use crate::repository::RepositoryPaginate;
use crate::repository_paginate;
use crate::service::schema_service::SchemaService;

repository_paginate!(SchemaService, SchemaModel, "schemas");

#[async_trait]
pub trait SchemaRepositoryTrait {
    async fn find_by_slug(&self, slug: &str) -> Result<SchemaModel>;
    async fn can_create(&self, slug: &str) -> Result<bool>;
    async fn create(&self, slug: &str, model: SchemaCreateModel) -> Result<SchemaModel>;
    async fn delete(&self, slug: &str) -> Result<()>;
    async fn update(&self, slug: &str, model: SchemaUpdateModel) -> Result<SchemaModel>;
    async fn update_fields(&self, slug: &str, model: SchemaFieldsModel) -> Result<SchemaModel>;
    async fn get_fields(&self, slug: &str) -> Result<SchemaFieldsModel>;
}

#[async_trait]
impl SchemaRepositoryTrait for SchemaService {
    async fn find_by_slug(
        &self,
        slug: &str,
    ) -> Result<SchemaModel> {
        let result: Option<SchemaModel> = self.db.query(r#"
            SELECT * FROM schemas WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn can_create(
        &self,
        slug: &str,
    ) -> Result<bool> {
        let result: Option<String> = self.db.query(r#"
            SELECT slug FROM schemas WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .await?
            .take(0)?;

        match result {
            Some(..) => Ok(false),
            None => Ok(true)
        }
    }

    async fn create(
        &self,
        slug: &str,
        model: SchemaCreateModel,
    ) -> Result<SchemaModel> {
        if !self.can_create(slug).await? {
            Err(ApiError::from(DbError::EntryAlreadyExists))?
        }

        let result: Option<SchemaModel> = self.db.query(r#"
            BEGIN TRANSACTION;

            CREATE schemas CONTENT {
	            slug: $slug,
	            title: $title,
	            is_collection: $is_collection
            };

            CREATE permissions CONTENT {
                id: $permission_read_id,
                slug: $permission_read
            };

            CREATE permissions CONTENT {
                id: $permission_write_id,
                slug: $permission_write
            };

            CREATE permissions CONTENT {
                id: $permission_delete_id,
                slug: $permission_delete
            };

            COMMIT TRANSACTION;
            "#)
            .bind(("slug", slug))
            .bind(("title", &model.title))
            .bind(("is_collection", &model.is_collection))
            .bind(("permission_read_id", format!("{}_read", slug)))
            .bind(("permission_read", format!("{}::read", slug)))
            .bind(("permission_write_id", format!("{}_write", slug)))
            .bind(("permission_write", format!("{}::write", slug)))
            .bind(("permission_delete_id", format!("{}_delete", slug)))
            .bind(("permission_delete", format!("{}::delete", slug)))
            .await?
            .take(0)?;

        match result {
            Some(value) => {
                if model.is_collection {
                    self.db.query(format!(r#"
                    BEGIN TRANSACTION;

                    DEFINE TABLE {0} SCHEMAFULL;
                    DEFINE FIELD slug ON TABLE {0} TYPE string;
                    DEFINE FIELD fields ON TABLE {0} FLEXIBLE TYPE option<object>;
                    DEFINE FIELD created_at ON TABLE {0} TYPE datetime DEFAULT time::now();
                    DEFINE FIELD updated_at ON TABLE {0} TYPE datetime VALUE time::now();
                    DEFINE INDEX idx_{0}_slug ON TABLE {0} COLUMNS slug UNIQUE;

                    COMMIT TRANSACTION;
                    "#, slug)).await?;
                } else {
                    self.db.query(r#"
                    CREATE singles CONTENT {
	                    slug: $slug,
                    };
                    "#)
                        .bind(("slug", slug))
                        .await?;
                }
                self.db.query(format!(r#"
                    BEGIN TRANSACTION;

                    RELATE roles:administrator->role_permissions->permissions:{0}_read;
                    RELATE roles:administrator->role_permissions->permissions:{0}_write;
                    RELATE roles:administrator->role_permissions->permissions:{0}_delete;

                    COMMIT TRANSACTION;
                    "#, slug)).await?;
                Ok(value)
            }
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }

    async fn delete(
        &self,
        slug: &str,
    ) -> Result<()> {
        let model = self.find_by_slug(slug).await?;

        if model.is_system {
            Err(ApiError::from(DbError::EntryDelete))?
        }

        self.db.query(r#"
            BEGIN TRANSACTION;

            DELETE FROM schemas WHERE slug=$slug;
            DELETE FROM singles WHERE slug=$slug;

            DELETE FROM permissions WHERE slug=$permission_read;
            DELETE FROM permissions WHERE slug=$permission_write;
            DELETE FROM permissions WHERE slug=$permission_delete;

            COMMIT TRANSACTION;
            "#)
            .bind(("slug", &model.slug))
            .bind(("permission_read", format!("{}::read", &model.slug)))
            .bind(("permission_write", format!("{}::write", &model.slug)))
            .bind(("permission_delete", format!("{}::delete", &model.slug)))
            .await?;

        if model.is_collection {
            self.db.query(format!(r#"
                REMOVE TABLE IF EXISTS {};
                "#, &model.slug))
                .await?;
        }

        Ok(())
    }

    async fn update(
        &self,
        slug: &str,
        model: SchemaUpdateModel,
    ) -> Result<SchemaModel> {
        let result: Option<SchemaModel> = self.db.query(r#"
            UPDATE schemas MERGE {
                title: $title
            } WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .bind(("title", model.title))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryUpdate))
        }
    }

    async fn update_fields(
        &self,
        slug: &str,
        model: SchemaFieldsModel,
    ) -> Result<SchemaModel> {
        let result: Option<SchemaModel> = self.db.query(r#"
            UPDATE schemas MERGE {
                fields: $fields
            } WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .bind(("fields", model.fields))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryUpdate))
        }
    }

    async fn get_fields(
        &self,
        slug: &str,
    ) -> Result<SchemaFieldsModel> {
        let result: Option<SchemaModel> = self.db.query(r#"
            SELECT * FROM schemas WHERE slug=$slug;
            "#)
            .bind(("slug", slug))
            .await?
            .take(0)?;

        match result {
            Some(schema_model) => Ok(SchemaFieldsModel { fields: schema_model.fields }),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }
}
