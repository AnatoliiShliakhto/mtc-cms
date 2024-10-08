use super::*;

pub async fn find_system_info_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_READ).await?;

    let system_info = state.repository.find_system_info().await?;
    let migrations = state.repository.find_migrations().await?;
    let mut json_obj = json!({});
    json_obj.insert_value("info", json!(system_info));
    json_obj.insert_value("migrations", json!(migrations));

    json_obj.to_response()
}