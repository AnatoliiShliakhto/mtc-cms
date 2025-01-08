use super::*;

/// Initiates the sign-out process when triggered by [`Event`].
///
/// This function sends a delete request to the authentication API
/// to sign the user out. Upon successful sign-out, it refreshes the
/// application state using the `SyncAction::RefreshState` action.
///
/// # Arguments
///
/// * `event` - The mouse event that triggers the sign-out action.
pub fn sign_out_task(event: Event<MouseData>) {
    let sync = use_coroutine_handle::<SyncAction>();

    spawn(async move {
        if delete_request!(url!(API_AUTH)) {
            sync.send(SyncAction::RefreshState())
        }
    });
}