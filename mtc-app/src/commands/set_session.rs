use super::*;

/// Set the current session header in each request.
///
/// This command sets the session to the specified `session` string. The
/// session is stored in the application state and used to authenticate
/// all requests to the server.
///
/// # Arguments
///
/// - `session`: The session string to set.
///
/// # Errors
///
/// Returns a `tide::Error` if the session string is invalid.
#[command(async, rename_all = "snake_case")]
pub async fn set_session(
    state: State<'_, AppState>,
    session: String,
) -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert("session", session.parse().unwrap());
    let client = ClientBuilder::default()
        .default_headers(headers)
        .build()?;

    let mut state_client = state.client.lock().await;
    *state_client = client;

    Ok(())
}