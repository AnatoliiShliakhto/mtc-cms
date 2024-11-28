use super::*;

pub async fn find_user_list_handler(
    path: Option<Path<(Cow<'static, str>, bool)>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_READ).await?;

    let access = session.get_access_state().await?;

    let users = if let Some(Path((login, archive))) = path {
        state
            .repository
            .find_user_list(access, Some(login.to_uppercase().into()), Some(archive))
            .await?
    } else {
        state.repository.find_user_list(access, None, None).await?
    };

    users.to_response()
}

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

pub async fn delete_user_handler(
    Path(id): Path<Cow<'static, str>>,
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_DELETE).await?;

    state.repository.delete_user(id).await?;

    Ok(StatusCode::OK)
}

pub async fn check_users_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    session.has_permission(PERMISSION_USERS_READ).await?;

    let access = session.get_access_state().await?;

    let users_details = state.repository.check_users(
        payload.self_obj::<Vec<Cow<'static, str>>>().unwrap_or_default(),
        access,
    ).await?;

    users_details.to_response()
}

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

fn generate_password(len: usize) -> Cow<'static, str> {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghjkmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    let one_char = || CHARSET[rng.random_range(0..CHARSET.len())] as char;
    std::iter::repeat_with(one_char).take(len).collect()
}