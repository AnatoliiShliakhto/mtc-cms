use crate::error::api_error::ApiError;
use crate::repository::permissions_repository::PermissionsRepository;
use crate::repository::role_repository::RoleRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::permissions_service::PermissionsService;
use crate::service::role_service::RoleService;
use crate::service::user_service::UserService;

pub struct AppState {
    pub role_service: RoleService,
    pub user_service: UserService,
    pub permissions_service: PermissionsService,
}

impl AppState {
    pub async fn new() -> Result<AppState, ApiError> {
        let role_service = RoleService::new(RoleRepository)?;
        let user_service = UserService::new(UserRepository)?;
        let permissions_service = PermissionsService::new(PermissionsRepository)?;


        Ok(Self {
            role_service,
            user_service,
            permissions_service,
        })
    }
}
