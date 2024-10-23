use super::*;

pub async fn sign_in_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let Some(login) = payload.key_str("login") else {
        Err(GenericError::BadRequest)?
    };
    let login: Cow<'static, str> = login.to_uppercase().into();


    let Some(password) = payload.key_str("password") else {
        Err(GenericError::BadRequest)?
    };

    let Ok(user) = state
        .repository
        .find_user_by_login(
            login.clone(),
            Access {
                level: -1,
                full: true,
            },
        )
        .await else { Err(SessionError::InvalidCredentials)? };

    if user.blocked {
        Err(SessionError::UserBlocked)?
    }

    let argon2 = Argon2::default();

    let Ok(password_hash) = PasswordHash::new(&user.password) else {
        Err(SessionError::PasswordHash)?
    };

    if argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_err()
    {
        Err(SessionError::InvalidCredentials)?
    }

    let auth_state = AuthState {
        id: user.id,
        roles: state.repository.find_roles_by_login(login.clone()).await?.into_iter().collect(),
        group: state.repository.find_group_by_login(login.clone()).await?,
        permissions: state.repository
            .find_permissions_by_login(login.clone()).await?.into_iter().collect(),
    };
    let access = if auth_state.is_admin() {
        Access { level: -1, full: true }
    } else {
        state.repository.find_user_access(login.clone()).await?
    };

    state.repository.increment_user_access_count(login).await?;
    session.sign_in(&auth_state, &user.login, &access).await?;

    Ok(StatusCode::OK)
}

pub async fn sign_out_handler(
    session: Session,
) -> Result<impl IntoResponse> {
    session.clear().await;

    Ok(StatusCode::OK)
}

pub async fn change_password_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let auth_state = session.get_auth_state().await?;
    if !auth_state.is_authenticated() {
        Err(SessionError::AccessForbidden)?
    }

    let Some(current_password) = payload.key_str("current_password") else {
        Err(GenericError::BadRequest)?
    };
    let Some(new_password) = payload.key_str("new_password") else {
        Err(GenericError::BadRequest)?
    };

    let Ok(user) = state
        .repository
        .find_user(auth_state.id, Access{ level: -1, full: true })
        .await else { Err(SessionError::InvalidCredentials)? };

    if user.blocked {
        Err(SessionError::UserBlocked)?
    }

    let argon2 = Argon2::default();

    let Ok(password_hash) = PasswordHash::new(&current_password) else {
        Err(SessionError::PasswordHash)?
    };
    if argon2.verify_password(current_password.as_bytes(), &password_hash).is_err() {
        Err(SessionError::InvalidCredentials)?
    }

    let Ok(salt) = SaltString::from_b64(&state.config.password_salt) else {
        Err(SessionError::PasswordHash)?
    };
    let Ok(password_hash) = argon2.hash_password(new_password.as_bytes(), &salt) else {
        Err(SessionError::PasswordHash)?
    };

    state.repository.set_user_password(user.id, password_hash.to_string().into()).await?;

    Ok(StatusCode::OK)
}
