use axum::async_trait;

use mtc_model::list_model::RecordListModel;
use mtc_model::record_model::RecordModel;
use mtc_model::schema_model::{
    SchemaCreateModel, SchemaFieldsModel, SchemaModel, SchemaUpdateModel,
};

use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::Result;
use crate::repository::RepositoryPaginate;
use crate::repository_paginate;
use crate::service::schema_service::SchemaService;

repository_paginate!(SchemaService, SchemaModel, "schemas");

#[async_trait]
pub trait SchemaRepositoryTrait {
    async fn find_by_slug(&self, slug: &str) -> Result<SchemaModel>;
    async fn can_create(&self, slug: &str) -> Result<()>;
    async fn create(&self, auth: &str, slug: &str, model: SchemaCreateModel)
        -> Result<SchemaModel>;
    async fn delete(&self, slug: &str) -> Result<()>;
    async fn update(&self, auth: &str, slug: &str, model: SchemaUpdateModel)
        -> Result<SchemaModel>;
    async fn update_fields(
        &self,
        auth: &str,
        slug: &str,
        model: SchemaFieldsModel,
    ) -> Result<SchemaModel>;
    async fn get_fields(&self, slug: &str) -> Result<SchemaFieldsModel>;
    async fn get_all_collections(&self) -> Result<RecordListModel>;
}

#[async_trait]
impl SchemaRepositoryTrait for SchemaService {
    async fn find_by_slug(&self, slug: &str) -> Result<SchemaModel> {
        self.db
            .query(
                r#"
            SELECT * FROM schemas WHERE slug=$slug;
            "#,
            )
            .bind(("slug", slug))
            .await?
            .take::<Option<SchemaModel>>(0)?
            .ok_or(DbError::EntryNotFound.into())
    }

    async fn can_create(&self, slug: &str) -> Result<()> {
        let result: Option<String> = self
            .db
            .query(
                r#"
            SELECT slug FROM schemas WHERE slug=$slug;
            "#,
            )
            .bind(("slug", slug))
            .await?
            .take(0)?;

        match result {
            Some(..) => Err(DbError::EntryAlreadyExists.into()),
            None => Ok(()),
        }
    }

    async fn create(
        &self,
        auth: &str,
        slug: &str,
        model: SchemaCreateModel,
    ) -> Result<SchemaModel> {
        self.can_create(slug).await?;

        let sql = r#"
            BEGIN TRANSACTION;

            CREATE schemas CONTENT {
	            slug: $slug,
	            title: $title,
	            fields: $fields,
	            is_collection: $is_collection,
	            is_public: $is_public,
	            created_by: $auth_id,
	            updated_by: $auth_id
            };
        "#;

        let result: Option<SchemaModel> = self
            .db
            .query(match model.is_public {
                true => [sql, "COMMIT TRANSACTION;"].concat(),
                false => [
                    sql,
                    r#"
                    CREATE permissions CONTENT {
                        id: $permission_read_id,
                        slug: $permission_read,
                        created_by: $auth_id
                    };

                    CREATE permissions CONTENT {
                        id: $permission_write_id,
                        slug: $permission_write,
                        created_by: $auth_id
                    };

                    CREATE permissions CONTENT {
                        id: $permission_delete_id,
                        slug: $permission_delete,
                        created_by: $auth_id
                    };

                    COMMIT TRANSACTION;
                "#,
                ]
                .concat(),
            })
            .bind(("auth_id", auth))
            .bind(("slug", slug))
            .bind(("title", &model.title))
            .bind(("fields", &model.fields))
            .bind(("is_collection", &model.is_collection))
            .bind(("is_public", &model.is_public))
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
                    self.db
                        .query(format!(
                            r#"
                        BEGIN TRANSACTION;

                        DEFINE TABLE {0} SCHEMAFULL;
                        DEFINE FIELD slug ON TABLE {0} TYPE string;
                        DEFINE FIELD title ON TABLE {0} TYPE string;
                        DEFINE FIELD fields ON TABLE {0} FLEXIBLE TYPE option<object>;
                        DEFINE FIELD published ON TABLE {0} TYPE bool DEFAULT false;
                        DEFINE FIELD created_at ON TABLE {0} TYPE datetime DEFAULT time::now();
                        DEFINE FIELD updated_at ON TABLE {0} TYPE datetime VALUE time::now();
                        DEFINE FIELD created_by ON TABLE {0} TYPE string;
                        DEFINE FIELD updated_by ON TABLE {0} TYPE string;
                        DEFINE INDEX idx_{0}_update ON TABLE {0} COLUMNS updated_at;
                        DEFINE INDEX idx_{0}_slug ON TABLE {0} COLUMNS slug UNIQUE;

                        COMMIT TRANSACTION;
                        "#,
                            slug
                        ))
                        .await?;
                } else {
                    self.db
                        .query(
                            r#"
                        CREATE singles CONTENT {
	                        slug: $slug,
	                        title: $title,
	                        created_by: $auth_id,
	                        updated_by: $auth_id
                        };
                        "#,
                        )
                        .bind(("auth_id", auth))
                        .bind(("title", &model.title))
                        .bind(("slug", slug))
                        .await?;
                }
                if !model.is_public {
                    self.db
                        .query(format!(
                            r#"
                        BEGIN TRANSACTION;

                        RELATE roles:administrator->role_permissions->permissions:{0}_read;
                        RELATE roles:administrator->role_permissions->permissions:{0}_write;
                        RELATE roles:administrator->role_permissions->permissions:{0}_delete;

                        COMMIT TRANSACTION;
                        "#,
                            slug
                        ))
                        .await?;
                }
                Ok(value)
            }
            _ => Err(DbError::EntryAlreadyExists.into()),
        }
    }

    async fn delete(&self, slug: &str) -> Result<()> {
        let model = self.find_by_slug(slug).await?;

        if model.is_system {
            Err(ApiError::from(DbError::EntryDelete))?
        }

        self.db
            .query(
                r#"
            BEGIN TRANSACTION;

            DELETE FROM schemas WHERE slug=$slug;
            DELETE FROM singles WHERE slug=$slug;

            DELETE FROM permissions WHERE slug=$permission_read;
            DELETE FROM permissions WHERE slug=$permission_write;
            DELETE FROM permissions WHERE slug=$permission_delete;

            COMMIT TRANSACTION;
            "#,
            )
            .bind(("slug", &model.slug))
            .bind(("permission_read", format!("{}::read", &model.slug)))
            .bind(("permission_write", format!("{}::write", &model.slug)))
            .bind(("permission_delete", format!("{}::delete", &model.slug)))
            .await?;

        if model.is_collection {
            self.db
                .query(format!(
                    r#"
                REMOVE TABLE IF EXISTS {};
                "#,
                    &model.slug
                ))
                .await?;
        }

        Ok(())
    }

    async fn update(
        &self,
        auth: &str,
        slug: &str,
        model: SchemaUpdateModel,
    ) -> Result<SchemaModel> {
        self.db
            .query(
                r#"
            UPDATE schemas MERGE {
                title: $title,
                fields: $fields,
                updated_by: $auth_id
            } WHERE slug=$slug;
            "#,
            )
            .bind(("auth_id", auth))
            .bind(("slug", slug))
            .bind(("title", model.title))
            .bind(("fields", model.fields))
            .await?
            .take::<Option<SchemaModel>>(0)?
            .ok_or(DbError::EntryUpdate.into())
    }

    async fn update_fields(
        &self,
        auth: &str,
        slug: &str,
        model: SchemaFieldsModel,
    ) -> Result<SchemaModel> {
        self.db
            .query(
                r#"
            UPDATE schemas MERGE {
                fields: $fields,
                updated_by: $auth_id
            } WHERE slug=$slug;
            "#,
            )
            .bind(("auth_id", auth))
            .bind(("slug", slug))
            .bind(("fields", model.fields))
            .await?
            .take::<Option<SchemaModel>>(0)?
            .ok_or(DbError::EntryUpdate.into())
    }

    async fn get_fields(&self, slug: &str) -> Result<SchemaFieldsModel> {
        let result: Option<SchemaModel> = self
            .db
            .query(
                r#"
            SELECT * FROM schemas WHERE slug=$slug;
            "#,
            )
            .bind(("slug", slug))
            .await?
            .take(0)?;

        match result {
            Some(schema_model) => Ok(SchemaFieldsModel {
                fields: schema_model.fields,
            }),
            _ => Err(ApiError::from(DbError::EntryNotFound)),
        }
    }

    async fn get_all_collections(&self) -> Result<RecordListModel> {
        Ok(RecordListModel {
            list: self
            .db
            .query(r#"SELECT slug, title FROM schemas WHERE is_collection = true AND is_system = false;"#)
            .await?
            .take::<Vec<RecordModel>>(0)?
        })
    }
}
