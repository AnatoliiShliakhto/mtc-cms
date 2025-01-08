use super::*;

/// Return a list of users.
///
/// If `login` is provided, return all users which have a login
/// containing the given string. If `archive` is true, return archived
/// users, otherwise return active users.
///
/// The [`PERMISSION_USERS_READ`] permission is required.
///
/// This API endpoint is used to populate the user list in the MTC web
/// interface.
///
/// The response is a JSON array containing [`Entry`] objects
pub async fn find_user_list_handler(
    path: Option<Path<(Cow<'static, str>, bool)>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_READ).await?;

    let access = session.get_access_state().await?;

    if let Some(Path((login, archive))) = path {
        state
            .repository
            .find_user_list(access, Some(login.to_uppercase().into()), Some(archive))
            .await?
    } else {
        state.repository.find_user_list(access, None, None).await?
    }.to_response()
}

/// Return the specified user.
///
/// If `id` is [`ID_CREATE`], return an empty user object with the
/// `created_by` and `updated_by` fields set to the current user.
/// Otherwise, return the user with the given `id`. The
/// [`PERMISSION_USERS_READ`] permission is required.
///
/// The response is a JSON object containing the [`User`] data and a
/// `roles` field containing a JSON array of role IDs.
///
/// This API endpoint is used to populate the user creation and edition
/// forms in the MTC web interface.
pub async fn find_user_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_READ).await?;

    let user = session.get_auth_login().await?;
    let access = session.get_access_state().await?;

    let user = if id.eq(ID_CREATE) {
        User {
            created_by: user.clone(),
            updated_by: user,
            ..Default::default()
        }
    } else {
        state.repository.find_user(id.clone(), access).await?
    };

    let user_roles = state.repository.find_roles_ids_by_user_id(id).await?;
    let mut json_obj = json!(user);
    json_obj.insert_value("roles", json!(user_roles));

    json_obj.to_response()
}

/// Update the specified user.
///
/// The [`PERMISSION_USERS_WRITE`] permission is required.
///
/// The request body must contain a JSON object with the user data.
/// The `id` field is required to identify the user to update.
///
/// The response is a `200 status code` on successful update.
pub async fn update_user_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_WRITE).await?;

    if payload.is_null() {
        Err(GenericError::BadRequest)?
    }

    let by = session.get_auth_login().await?;

    state.repository.update_user(payload, by).await?;

    Ok(StatusCode::OK)
}

/// Delete a user by ID.
///
/// # Authorization
///
/// The user must have the [`PERMISSION_USERS_DELETE`] permission.
///
/// # Arguments
///
/// * `id`: The ID of the user to delete.
/// * `state`: The shared application state, including the repository for database interactions.
/// * `session`: The current user session, used for permission checks.
///
/// # Response
///
/// Returns a `200 OK status code` if the user was deleted successfully.
///
/// # Errors
///
/// Returns an error if the user does not have the required permission or if the deletion fails.
pub async fn delete_user_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_DELETE).await?;

    state.repository.delete_user(id).await?;

    Ok(StatusCode::OK)
}

/// Check if the specified users exist.
///
/// The [`PERMISSION_USERS_READ`] permission is required.
///
/// The request body must contain a JSON array of user IDs to check.
///
/// The response is a JSON array of booleans, indicating whether the specified user
/// exists or not.
///
/// The response is a JSON array of the same length as the input array, with each
/// element indicating whether the user at that index exists or not. If the user
/// does not exist, the corresponding element is `false`, otherwise it is `true`.
pub async fn check_users_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_READ).await?;

    let access = session.get_access_state().await?;

    state.repository.check_users(
        payload.self_obj::<Vec<Cow<'static, str>>>().unwrap_or_default(),
        access,
    ).await?
        .to_response()
}

/// Process multiple users at once.
///
/// The [`PERMISSION_USERS_WRITE`] permission is required.
///
/// The request body must contain a JSON object with the following keys:
///
/// * `logins`: An array of user logins to process.
/// * `block`: A boolean indicating whether the users should be blocked.
/// * `reassign`: A boolean indicating whether the `roles` field should be replaced
///   or appended to the existing roles.
/// * `recreate`: A boolean indicating whether the users should be recreated if they
///   do not exist.
/// * `group`: The group to assign to the users.
/// * `roles`: An array of role IDs to assign to the users.
///
/// The response is an array of [`UserDetailsDto`] objects, each describing the user
/// that was processed.
///
/// The response is a JSON array of the same length as the input array, with each
/// element describing the user at that index. If the user does not exist, the
/// corresponding element is omitted from the response.
pub async fn process_users_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_WRITE).await?;

    let access = session.get_access_state().await?;
    let by = session.get_auth_login().await?;

    let logins = payload
        .key_obj::<Vec<Cow<'static, str>>>("logins").unwrap_or_default();
    let block = payload.key_bool("block").unwrap_or(false);
    let reassign = payload.key_bool("reassign").unwrap_or(false);
    let recreate = payload.key_bool("recreate").unwrap_or(false);
    let group = payload.key_str("group").unwrap_or_default();
    let roles = payload
        .key_obj::<Vec<Cow<'static, str>>>("roles").unwrap_or_default();

    let mut users_details = vec![];

    for login in logins {
        let user = match state
            .repository
            .find_user_by_login(login.clone(), Access::administrator())
            .await {
            Ok(user) => user,
            _ => User::default(),
        };
        if user.access_level < access.level { continue }
        let mut payload = json!({
            "id": user.id,
            "login": login,
            "group": group,
            "blocked": block
        });
        let mut user_roles = if !user.id.is_empty(){
            state.repository.find_roles_ids_by_user_id(user.id.clone()).await.unwrap_or_default()
        } else {
            vec![]
        };
        if !reassign {
            roles.iter().for_each(|role| { user_roles.push(role.clone()) })
        } else {
            user_roles = roles.clone()
        }
        payload.insert_value("roles", json!(user_roles));
        let mut password = Cow::Borrowed("".into());
        if recreate | user.id.is_empty() {
            password = generate_password(8);
            payload.insert_value("password", Value::String(password.clone().into()));
        }
        if state.repository.update_user(payload, by.clone()).await.is_ok() {
            users_details.push(UserDetailsDto {
                id: user.id,
                login: login.clone(),
                group: group.clone(),
                password,
                blocked: block,
                last_access: user.last_access.as_ref().clone(),
                access_count: user.access_count,
            })
        }
    }

    users_details.to_response()
}

/// Generates a random password of the specified length and symbols.
///
/// # Examples
///
/// ```rust
///     let password = generate_password(8);
///     println!("Generated password: {}", password);
/// ```
fn generate_password(len: usize) -> Cow<'static, str> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghjkmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    let one_char = || CHARSET[rng.random_range(0..CHARSET.len())] as char;
    std::iter::repeat_with(one_char).take(len).collect()
}