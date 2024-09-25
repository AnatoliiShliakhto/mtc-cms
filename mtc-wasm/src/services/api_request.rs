use super::*;

pub enum ApiRequestAction {
    PostThenBack(Cow<'static, str>, Option<Value>),
    DeleteThenMessage(Cow<'static, str>, Option<Value>),
    DeleteThenBack(Cow<'static, str>, Option<Value>),
}

pub async fn api_request_service(mut rx: UnboundedReceiver<ApiRequestAction>) {
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();

    while let Some(msg) = rx.next().await {
        match msg {
            ApiRequestAction::PostThenBack(url, value) =>
                request_post_then_back_task(url, value),
            ApiRequestAction::DeleteThenMessage(url, value) => {
                message_box_task.send(
                    MessageBoxAction::AlertDialog(
                        t!("message-confirm-deletion"),
                        request_delete_then_msg_task, (
                            url,
                            value,
                        ),
                    )
                )
            },
            ApiRequestAction::DeleteThenBack(url, value) => {
                message_box_task.send(
                    MessageBoxAction::AlertDialog(
                        t!("message-confirm-deletion"),
                        request_delete_then_back_task, (
                            url,
                            value,
                        ),
                    )
                )
            },
        }
    }
}
