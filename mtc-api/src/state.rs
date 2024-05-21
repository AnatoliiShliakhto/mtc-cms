use std::sync::Arc;

use crate::error::Result;
use crate::provider::config_provider::Config;
use crate::provider::database_provider::Database;
use crate::service::group_service::GroupService;
use crate::service::permissions_service::PermissionsService;
use crate::service::role_service::RoleService;
use crate::service::schema_service::SchemaService;
use crate::service::user_service::UserService;

pub struct AppState {
    pub cfg: Arc<Config>,
    pub db: Arc<Database>,

    pub schema_service: SchemaService,
    pub group_service: GroupService,
    pub role_service: RoleService,
    pub user_service: UserService,
    pub permissions_service: PermissionsService,
}

impl AppState {
    pub async fn new(cfg: Config, db: Database) -> Result<AppState> {
        let cfg = Arc::new(cfg);
        let db = Arc::new(db);

        let schema_service = SchemaService::new(&cfg, &db);
        let group_service = GroupService::new(&cfg, &db);
        let permissions_service = PermissionsService::new(&cfg, &db);
        let role_service = RoleService::new(&cfg, &db);
        let user_service = UserService::new(&cfg, &db);

        Ok(Self {
            cfg,
            db,

            schema_service,
            group_service,
            role_service,
            user_service,
            permissions_service,
        })
    }
}
