use crate::prelude::*;
use axum::routing::patch;

pub fn routes(
    state: Arc<AppState>
) -> Router {
    Router::new()
        .route("/storage/private/{path}", get(find_private_assets_handler)
            .post(private_upload_handler))
        .route("/storage/private/{path}/{file}", delete(delete_private_asset_handler))

        .route("/storage/public/{path}", get(find_public_assets_handler)
            .post(public_upload_handler))
        .route("/storage/public/{path}/{file}", delete(delete_public_asset_handler))

        .route("/contents/{slug}", get(find_content_list_handler))
        .route("/content/{schema}/{slug}", get(find_content_handler)
            .post(update_content_handler).delete(delete_content_handler))

        .route("/schemas", get(find_schema_list_handler))
        .route("/schema", post(update_schema_handler))
        .route("/schema/{id}", get(find_schema_handler)
            .delete(delete_schema_handler))

        .route("/personnel", post(check_users_handler))

        .route("/users/{login}/{archive}", get(find_user_list_handler))
        .route("/users", get(find_user_list_handler).post(process_users_handler))
        .route("/user", post(update_user_handler))
        .route("/user/{id}", get(find_user_handler)
            .delete(delete_user_handler)
        )

        .route("/roles", get(find_custom_role_list_handler))
        .route("/role", post(update_role_handler))
        .route("/role/{id}", get(find_role_handler)
            .delete(delete_role_handler)
        )

        .route("/groups", get(find_group_list_handler))
        .route("/group", post(update_group_handler))
        .route("/group/{id}", get(find_group_handler)
            .delete(delete_group_handler)
        )

        .route("/permissions", get(find_custom_permissions_handler))
        .route("/permission/{permission}", post(create_custom_permission_handler)
            .delete(delete_custom_permission_handler)
        )

        .route("/assets/course", post(find_course_files_handler))

        .route("/auth/qr", get(sign_in_qr_code_handler))
        .route("/auth", post(sign_in_handler)
            .delete(sign_out_handler)
            .patch(change_password_handler)
        )

        .route("/system/courses", post(course_files_update_handler))
        .route("/system/sitemap", post(sitemap_build_handler))
        .route("/system/rebuild", post(search_idx_rebuild_handler))
        .route("/system/migrate", post(migration_handler))
        .route("/system", get(find_system_info_handler))
        .route("/search", post(search_handler))
        .route("/sync", get(sync_handler))
        .route("/health", get(health_handler))
        .route(
            "/gate-passes", post(create_gate_pass_handler),
        )
        .route(
            "/gate-passes/imports", post(create_gate_passes_handler),
        )
        .route(
            "/gate-passes/exports", post(find_gate_passes_handler),
        )
        .route(
            "/gate-passes/searches", post(search_gate_passes_handler),
        )
        .route(
            "/gate-passes/syncs", post(find_sync_gate_passes_handler),
        )
        .route(
            "/gate-passes/{gate_pass_id}",
            patch(update_gate_pass_handler)
                .delete(delete_gate_pass_handler)
                .get(find_gate_pass_handler),
        )
        .route(
            "/gate-passes/{gate_pass_id}/blocks",
            post(update_gate_pass_block_handler),
        )
        .route(
            "/gate-passes/{gate_pass_id}/emails", post(send_gate_pass_email_handler),
        )
        .route(
            "/gate-passes/{gate_pass_id}/backs", get(generate_gate_pass_back_handler),
        )
        .route(
            "/gate-passes/{gate_pass_id}/fronts", get(generate_gate_pass_front_handler),
        )
        .route(
            "/gate-passes/{gate_pass_id}/validations", get(find_validation_gate_pass_handler))
        .with_state(state)
}