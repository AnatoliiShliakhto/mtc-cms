use crate::prelude::*;

pub fn routes(
    state: Arc<AppState>
) -> Router {
    Router::new()
        .route("/roles", get(find_role_list_handler))
        .route("/role/:id", get(find_role_by_slug_handler)
            .delete(delete_role_handler)
        )
        .route("/role", post(update_role_handler))

        .route("/groups", get(find_group_list_handler))
        .route("/group/:id", get(find_group_handler)
            .delete(delete_group_handler)
        )
        .route("/group", post(update_group_handler))

        .route("/permissions", get(find_custom_permissions_handler))
        .route("/permission/:permission", post(create_custom_permission_handler)
            .delete(delete_custom_permission_handler)
        )
        
        .route("/auth", post(sign_in_handler)
            .delete(sign_out_handler)
            .patch(change_password_handler)
        )
        
        .route("/sync", post(sync_handler))
        
        .with_state(state)
}