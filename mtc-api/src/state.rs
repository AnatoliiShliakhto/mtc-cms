use crate::error::Result;
use crate::repository::group_repository::GroupRepository;
use crate::repository::permissions_repository::PermissionsRepository;
use crate::repository::role_repository::RoleRepository;
use crate::repository::user_repository::UserRepository;
use crate::service::group_service::GroupService;
use crate::service::permissions_service::PermissionsService;
use crate::service::role_service::RoleService;
use crate::service::user_service::UserService;

pub struct AppState {
    pub group_service: GroupService,
    pub role_service: RoleService,
    pub user_service: UserService,
    pub permissions_service: PermissionsService,
}

impl AppState {
    pub async fn new() -> Result<AppState> {
        let group_service = GroupService::new(GroupRepository)?;
        let role_service = RoleService::new(RoleRepository)?;
        let user_service = UserService::new(UserRepository)?;
        let permissions_service = PermissionsService::new(PermissionsRepository)?;

        Ok(Self {
            group_service,
            role_service,
            user_service,
            permissions_service,
        })
    }
}
