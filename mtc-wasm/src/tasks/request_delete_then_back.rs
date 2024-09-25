use super::*;

pub fn request_delete_then_back_task(
    url: Cow<'static, str>,
    value: Option<Value>,
) {
    let message_box_task = use_coroutine_handle::<MessageBoxAction>();

    if let Some(value) = value {
        spawn(async move {
            match use_api_client()
                .delete(&*url)
                .json(&value)
                .send()
                .await
                .consume()
                .await {
                Ok(_) =>
                    if navigator().can_go_back() {
                        message_box_task.send(MessageBoxAction::Clear);
                        navigator().go_back()
                    } else {
                        message_box_task
                            .send(MessageBoxAction::Success(t!("message-success-deletion")))
                    }
                Err(e) =>
                    message_box_task
                        .send(MessageBoxAction::Error(e.message())),
            }
        });
    } else {
        spawn(async move {
            let message_box_task = use_coroutine_handle::<MessageBoxAction>();
            match use_api_client()
                .delete(&*url)
                .send()
                .await
                .consume()
                .await {
                Ok(_) =>
                    if navigator().can_go_back() {
                        message_box_task.send(MessageBoxAction::Clear);
                        navigator().go_back()
                    } else {
                        message_box_task
                            .send(MessageBoxAction::Success(t!("message-success-deletion")))
                    }
                Err(e) =>
                    message_box_task
                        .send(MessageBoxAction::Error(e.message())),
            }
        });
    }
}