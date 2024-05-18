/*
pub struct SingleTypeRepository;

repository_paginate!(SingleTypeRepository, SingleTypeModel, "single_types");

#[async_trait]
pub trait SingleTypeRepositoryTrait {
    async fn find(&self, id: &str) -> Result<SingleTypeModel>;
    async fn find_by_api(&self, api: &str) -> Result<SingleTypeModel>;
    async fn create(&self, model: SingleTypeCreateModel) -> Result<SingleTypeModel>;
    async fn delete(&self, api: &str) -> Result<()>;
}

#[async_trait]
impl SingleTypeRepositoryTrait for SingleTypeRepository {
    async fn find(
        &self,
        id: &str,
    ) -> Result<SingleTypeModel> {
        let result: Option<SingleTypeModel> = DB.query(r#"
            SELECT * FROM type::thing('single_types', $id);
            "#)
            .bind(("id", id.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn find_by_api(
        &self,
        api: &str,
    ) -> Result<SingleTypeModel> {
        let result: Option<SingleTypeModel> = DB.query(r#"
            SELECT * FROM single_types WHERE api=$api;
            "#)
            .bind(("name", api.to_string()))
            .await?
            .take(0)?;

        match result {
            Some(value) => Ok(value),
            _ => Err(ApiError::from(DbError::EntryNotFound))
        }
    }

    async fn create(&self, model: SingleTypeCreateModel) -> Result<SingleTypeModel> {
        if !can_api_create(&model.api).await? {
            Err(ApiError::from(DbError::EntryAlreadyExists))?
        }

        let result: Option<SingleTypeModel> = DB.query(r#"
            BEGIN TRANSACTION;

            CREATE single_types CONTENT {
	            api: $api,
            };

            CREATE tables CONTENT {
                name: $api,
                is_core: false
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
            .bind(("api", &model.api))
            .bind(("permission_read_id", format!("{}_read", &model.api)))
            .bind(("permission_read_name", format!("{}::read", &model.api)))
            .bind(("permission_write_id", format!("{}_write", &model.api)))
            .bind(("permission_write_name", format!("{}::write", &model.api)))
            .bind(("permission_delete_id", format!("{}_delete", &model.api)))
            .bind(("permission_delete_name", format!("{}::delete", &model.api)))
            .await?
            .take(0)?;

        match result {
            Some(value) => {
                DB.query(format!(r#"
                    BEGIN TRANSACTION;
                    DEFINE TABLE {0};
                    DEFINE FIELD created_at ON TABLE {0} TYPE datetime DEFAULT time::now();
                    DEFINE FIELD updated_at ON TABLE {0} TYPE datetime VALUE time::now();
                    COMMIT TRANSACTION;
                    "#, model.api)).await?;
                Ok(value)
            }
            _ => Err(ApiError::from(DbError::EntryAlreadyExists))
        }
    }

    async fn delete(&self, api: &str) -> Result<()> {
        if !can_api_delete(api).await? {
            Err(ApiError::from(DbError::EntryNotFound))?
        }

        DB.query(r#"
            BEGIN TRANSACTION;

            DELETE FROM single_types WHERE api=$api;
            DELETE FROM tables WHERE name=$api;

            DELETE type::thing('permissions', $permission_read_id);
            DELETE type::thing('permissions', $permission_write_id);
            DELETE type::thing('permissions', $permission_delete_id);

            COMMIT TRANSACTION;
            "#)
            .bind(("api", api))
            .bind(("permission_read_id", format!("{}_read", api)))
            .bind(("permission_write_id", format!("{}_write", api)))
            .bind(("permission_delete_id", format!("{}_delete", api)))
            .await?;

        DB.query(format!(r#"
                REMOVE TABLE IF EXISTS {};
                "#, api))
            .await?;

        Ok(())
    }
}

 */