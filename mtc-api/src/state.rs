use std::sync::Arc;

use crate::provider::config_provider::Config;
use crate::provider::database_provider::Database;
use crate::service::api_service::ApiService;
use crate::service::group_service::GroupService;
use crate::service::migration_service::MigrationService;
use crate::service::permissions_service::PermissionsService;
use crate::service::role_service::RoleService;
use crate::service::schema_service::SchemaService;
use crate::service::storage_service::StorageService;
use crate::service::system_service::SystemService;
use crate::service::user_service::UserService;

pub struct AppState {
    pub cfg: Arc<Config>,
    pub db: Arc<Database>,

    pub schema_service: SchemaService,
    pub group_service: GroupService,
    pub role_service: RoleService,
    pub user_service: UserService,
    pub permissions_service: PermissionsService,
    pub api_service: ApiService,
    pub storage_service: StorageService,
    pub migration_service: MigrationService,
    pub system_service: SystemService,
}

impl AppState {
    pub fn new(cfg: Config, db: Database) -> AppState {
        let cfg = Arc::new(cfg);
        let db = Arc::new(db);

        let schema_service = SchemaService::new(&cfg, &db);
        let group_service = GroupService::new(&cfg, &db);
        let permissions_service = PermissionsService::new(&cfg, &db);
        let role_service = RoleService::new(&cfg, &db);
        let user_service = UserService::new(&cfg, &db);
        let api_service = ApiService::new(&cfg, &db);
        let storage_service = StorageService::new(&cfg, &db);
        let migration_service = MigrationService::new(&cfg, &db);
        let system_service = SystemService::new(&cfg, &db);

        Self {
            cfg,
            db,

            schema_service,
            group_service,
            role_service,
            user_service,
            permissions_service,
            api_service,
            storage_service,
            migration_service,
            system_service,
        }
    }
}
