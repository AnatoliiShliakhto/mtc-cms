use super::*;

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