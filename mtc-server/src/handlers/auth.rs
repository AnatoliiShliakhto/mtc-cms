use super::*;

/// Handle a sign in request. This function can be called with a
/// user's login and password, or with an API key. The user's
/// authentication state is stored in the session.
///
/// # Errors
///
/// * `GenericError::BadRequest` if the request is malformed
/// * `SessionError::InvalidCredentials` if the user's login or password
///   is invalid
/// * `SessionError::UserBlocked` if the user is blocked
///
/// # Response
///
/// A successful response is a `200 OK status code`.
pub async fn sign_in_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let api_key = payload.key_str("api_key").unwrap_or_default();
    if !api_key.is_empty() {
        let re = regex::Regex::new(UUID_PATTERN).unwrap();
        if !re.is_match(&api_key) {
            return Err(GenericError::BadRequest)?
        }
    }

    let login: Cow<'static, str> =
        Cow::Owned(payload.key_str("login").unwrap_or_default().to_uppercase());
    let password = payload.key_str("password").unwrap_or_default();

    if !login.is_empty() & password.is_empty() { Err(GenericError::BadRequest)? }

    let Ok(user) = (if !login.is_empty() {
        state
            .repository
            .find_user_by_login(
                login.clone(),
                Access::administrator(),
            ).await
    } else {
        state
            .repository
            .find_user_by_api_key(
                api_key.clone()
            ).await
    }) else { Err(SessionError::InvalidCredentials)? };

    if user.blocked { Err(SessionError::UserBlocked)? }

    if !password.is_empty() {
        let argon2 = Argon2::default();

        let Ok(password_hash) = PasswordHash::new(&user.password) else {
            Err(SessionError::PasswordHash)?
        };

        if argon2
            .verify_password(password.as_bytes(), &password_hash)
            .is_err() { Err(SessionError::InvalidCredentials)? }
    }

    let user_state = state.repository.find_user_state(user.id.clone()).await?;

    let auth_state = AuthState::from(user_state.clone());

    let access = auth_state.is_admin()
        .then(|| Access::administrator())
        .unwrap_or(Access::from(user_state));

    state.repository.increment_user_access_count(login).await?;
    session.set_state(&auth_state, &access).await?;

    (!api_key.is_empty()).then(|| async {
        state.repository.update_user_api_key(
            user.id,
            api_key,
            session.get_session_id().to_string().into(),
            payload.key_str("os").unwrap_or_default(),
            payload.key_str("device").unwrap_or_default(),
        ).await.ok()
    });

    Ok(StatusCode::OK)
}

/// Clear the user session in the Database and log them out.
pub async fn sign_out_handler(
    session: Session,
) -> Result<impl IntoResponse> {
    session.clear();

    Ok(StatusCode::OK)
}

/// Handles the user's password change request.
///
/// This handler validates the current password and updates it with the new password provided
/// in the payload. The user must be authenticated and not blocked to perform this action.
///
/// # Arguments
///
/// * `state` - Shared application state containing configuration and repository access.
/// * `session` - User session for retrieving authentication state.
/// * `payload` - JSON payload containing `current_password` and `new_password` strings.
///
/// # Errors
///
/// Returns `SessionError::AccessForbidden` if the user is not authenticated.
/// Returns `GenericError::BadRequest` if required keys are missing from the payload.
/// Returns `SessionError::InvalidCredentials` if the current password is incorrect.
/// Returns `SessionError::UserBlocked` if the user is blocked.
/// Returns `SessionError::PasswordHash` if there is an error hashing the password.
///
/// # Response
///
/// Returns a 200 status code on successful password change.
pub async fn change_password_handler(
    state: State<Arc<AppState>>,
    session: Session,
    Payload(payload): Payload<Value>,
) -> Result<impl IntoResponse> {
    let auth_state = session.get_auth_state().await?;
    if !auth_state.is_authenticated() { Err(SessionError::AccessForbidden)? }

    let (Some(current_password), Some(new_password)) = (
        payload.key_str("current_password"),
        payload.key_str("new_password"),
    ) else {
        Err(GenericError::BadRequest)?
    };

    let Ok(user) = state
        .repository
        .find_user(auth_state.id, Access{ level: -1, full: true })
        .await else { Err(SessionError::InvalidCredentials)? };

    if user.blocked { Err(SessionError::UserBlocked)? }

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

/// Generates a QR code for user sign-in using an encrypted API key.
///
/// This handler retrieves the authenticated user's API key, encrypts it,
/// and generates a QR code in SVG format for sign-in purposes. The QR code
/// contains the encrypted API key prefixed with "MTC:000:".
///
/// # Arguments
///
/// * `state` - Shared application state containing configuration and repository access.
/// * `session` - User session for retrieving authentication state and user ID.
///
/// # Errors
///
/// Returns `SessionError::AccessForbidden` if the user is not authenticated.
///
/// # Response
///
/// Returns an SVG image with a content-type of "image/svg+xml" on success.
pub async fn sign_in_qr_code_handler(
    state: State<Arc<AppState>>,
    session: Session,
) -> Result<impl IntoResponse> {
    let auth_state = session.get_auth_state().await?;
    if !auth_state.is_authenticated() { Err(SessionError::AccessForbidden)? };

    let mcrypt = new_magic_crypt!(env!("CRYPT_KEY"), 256);
    let api_key = state
        .repository
        .find_api_key_by_user_id(session.get_auth_id().await?)
        .await?;
    let encrypted_api_key = mcrypt.encrypt_str_to_base64(api_key);
    let qr_str = format!(
        "MTC:000:{}",
        encrypted_api_key
    );

    let svg = qrcode_generator::to_svg_to_string(
        qr_str,
        qrcode_generator::QrCodeEcc::Low, 1024, None::<&str>
    ).unwrap_or_default();

    Ok(([("content-type", "image/svg+xml")], svg))
}
