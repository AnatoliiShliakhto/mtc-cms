use super::*;

pub fn change_password_task(event: Event<FormData>) {
    let sync_task = use_coroutine_handle::<SyncAction>();
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();
    let api_client = use_api_client();
    let mut busy = use_busy();

    let current_password =
        event.get_str("current-password").unwrap_or_default();
    let new_password =
        event.get_str("new-password").unwrap_or_default();
    let password_confirmation = 
        event.get_str("password-confirmation").unwrap_or_default();
    
    if new_password.ne(&password_confirmation) {
        message_box_task.send(MessageBoxAction::Error(t!("error-password-not-match")));
        return
    }
    
    spawn(async move {
        *busy.write() = true;

        match api_client()
            .patch([API_ENDPOINT, API_AUTH].join("/"))
            .json(&json!({
                "current_password": current_password, 
                "new_password": new_password
            }))
            .send()
            .await
            .consume()
            .await {
            Ok(_) => 
                message_box_task.send(MessageBoxAction::Success(t!("message-password-changed"))),
            Err(e) =>
                message_box_task.send(MessageBoxAction::Error(e.message())),
        }

        *busy.write() = false;
    });
}