use super::*;

/// Handler for `/system/info` route.
///
/// Returns information about the system in JSON format, including
/// system information, migrations, and sitemap.
///
/// Requires the [`PERMISSION_SCHEMAS_READ`] permission.
///
/// # Errors
///
/// Returns a `GenericError` if the permission check fails.
pub async fn find_system_info_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_SCHEMAS_READ).await?;

    let system_info = state.repository.find_system_info().await?;
    let migrations = state.repository.find_migrations().await?;
    let sitemap = state.repository.get_system_value("sitemap".into()).await?;
    let mut json_obj = json!({});

    json_obj.insert_value("info", json!(system_info));
    json_obj.insert_value("migrations", json!(migrations));
    json_obj.insert_value("sitemap", json!(sitemap));

    json_obj.to_response()
}

/// Handler for `/system/migrate` route.
///
/// Runs migrations for the current system.
///
/// If there are no existing migrations, this handler will create a new user
/// with the provided `login` and `password` and set up the migrations table.
///
/// If there are existing migrations, this handler will require the
/// [`PERMISSION_ADMINISTRATOR`] role to continue.
///
/// # Errors
///
/// Returns a `GenericError` if the permission check fails or if there is an
/// error running a migration.
pub async fn migration_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let mut migrations = state.repository.find_migrations().await?;
    let mut login = session.get_auth_login().await.unwrap_or_default();
    let mut password = Cow::Borrowed("");

    if migrations.is_empty() {
        let pwd = payload.key_str("password").unwrap_or(ROLE_ADMINISTRATOR.into());
        let salt = SaltString::from_b64(&state.config.password_salt).unwrap();

        let argon2 = Argon2::default();
        let Ok(password_hash) = argon2.hash_password(pwd.as_bytes(), &salt) else {
            Err(SessionError::PasswordHash)?
        };
        password = Cow::Owned(password_hash.to_string());
        login = payload.key_str("login").unwrap_or(ROLE_ADMINISTRATOR.into());
    } else if !session.get_auth_state().await?.has_role(ROLE_ADMINISTRATOR) {
        Err(SessionError::AccessForbidden)?
    }

    let migration_files = state.repository.get_migration_files().await?
        .iter()
        .filter(|value| !migrations.contains(value.as_ref()))
        .cloned()
        .collect::<BTreeSet<Cow<'static, str>>>();

    for file in migration_files.iter() {
        let sql = state.repository.get_migration_file(file).await?;

        state.repository.migrate(sql, login.clone(), password.clone()).await?;
        info!("Migration {} is done!", file);

        migrations.insert(file.clone());

        state.repository.update_migrations(migrations.clone()).await?;
    }

    Ok(StatusCode::OK)
}

/// Handler for `/system/search_idx_rebuild` route.
///
/// Rebuilds the search index.
///
/// Requires the [`PERMISSION_ADMINISTRATOR`] role.
///
/// # Errors
///
/// Returns a `SessionError` if the permission check fails.
pub async fn search_idx_rebuild_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    if !session.get_auth_state().await?.has_role(ROLE_ADMINISTRATOR) {
        Err(SessionError::AccessForbidden)?
    }

    state.repository.rebuild_search_idx().await?;

    Ok(())
}

/// Handler for `/system/search` route.
///
/// Searches for content that matches the provided search query.
///
/// The search query is sanitized to remove any non-alphanumeric characters
/// except for whitespace, hyphens, and underscores.
///
/// The search results include only records that the current user has access to.
///
/// # Errors
///
/// Returns a `GenericError` if the search query is empty or if there is an error
/// running the search query.
pub async fn search_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let Some(payload) = payload.as_str() else {
        Err(GenericError::BadRequest)?
    };
    let re = regex::Regex::new(r"[^a-zA-Zа-яА-ЯіїєґІЇЄҐ0-9 -_]").unwrap();
    let payload = re.replace_all(payload, "").to_string();

    let auth_state = session
        .get_auth_state()
        .await
        .unwrap_or_default();

    let user_custom_permissions = auth_state
        .permissions
        .iter()
        .map(|permission| permission
            .split_once("::")
            .unwrap_or((PERMISSION_PUBLIC, "")).0.to_owned().into())
        .collect::<BTreeSet<Cow<'static, str>>>();

    let search_idx = state
        .repository
        .search_content(payload.into(), user_custom_permissions)
        .await?;

    search_idx.to_response()
}

/// Handler for `/system/sitemap_build` route.
///
/// Builds the sitemap for the system.
///
/// Requires the [`PERMISSION_ADMINISTRATOR`] role.
///
/// # Errors
///
/// Returns a `SessionError` if the permission check fails or if there is an
/// error building the sitemap.
pub async fn sitemap_build_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    if !session.get_auth_state().await?.has_role(ROLE_ADMINISTRATOR) {
        Err(SessionError::AccessForbidden)?
    }

    state.repository.sitemap_build().await?;

    Ok(())
}

/// Handler for `/system/course_files_update` route.
///
/// Updates the `course_files` collection.
///
/// Requires the [`PERMISSION_ADMINISTRATOR`] role.
///
/// # Errors
///
/// Returns a `SessionError` if the permission check fails or if there is an
/// error updating the `course_files` collection.
pub async fn course_files_update_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    if !session.get_auth_state().await?.has_role(ROLE_ADMINISTRATOR) {
        Err(SessionError::AccessForbidden)?
    }

    state.repository.drop_course_files().await?;

    let courses = state.repository.find_content_list("course".into(), true).await?;

    for course in courses.iter() {
        let files = state.repository.get_course_files(course.slug.clone()).await?;
        state.repository.update_course_files(course.slug.clone(), files).await?;
    }
    Ok(())
}