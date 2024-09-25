use super::*;

pub enum MessageBoxAction {
    Clear,
    Alert(Cow<'static, str>),
    Info(Cow<'static, str>),
    Error(Cow<'static, str>),
    Success(Cow<'static, str>),
    Warning(Cow<'static, str>),
    
    AlertDialog(Cow<'static, str>, MessageBoxFn, MessageBoxFnArgs),
    InfoDialog(Cow<'static, str>, MessageBoxFn, MessageBoxFnArgs),
    ErrorDialog(Cow<'static, str>, MessageBoxFn, MessageBoxFnArgs),
    SuccessDialog(Cow<'static, str>, MessageBoxFn, MessageBoxFnArgs),
    WarningDialog(Cow<'static, str>, MessageBoxFn, MessageBoxFnArgs),
}

pub async fn message_box_service(mut rx: UnboundedReceiver<MessageBoxAction>) {
    let message_box = use_message_box();

    while let Some(msg) = rx.next().await {
        match msg {
            MessageBoxAction::Clear =>
                *message_box.write_unchecked() = None,
            MessageBoxAction::Alert(message) =>
                *message_box.write_unchecked() = Some((MessageKind::Alert, message, None, None)),
            MessageBoxAction::Info(message) =>
                *message_box.write_unchecked() = Some((MessageKind::Info, message, None, None)),
            MessageBoxAction::Error(message) =>
                *message_box.write_unchecked() = Some((MessageKind::Error, message, None, None)),
            MessageBoxAction::Success(message) =>
                *message_box.write_unchecked() = Some((MessageKind::Success, message, None, None)),
            MessageBoxAction::Warning(message) =>
                *message_box.write_unchecked() = Some((MessageKind::Warning, message, None, None)),

            MessageBoxAction::AlertDialog(message, task, args) =>
                *message_box.write_unchecked() = Some((MessageKind::Alert, message, Some(task), Some(args))),
            MessageBoxAction::InfoDialog(message, task, args) =>
                *message_box.write_unchecked() = Some((MessageKind::Info, message, Some(task), Some(args))),
            MessageBoxAction::ErrorDialog(message, task, args) =>
                *message_box.write_unchecked() = Some((MessageKind::Error, message, Some(task), Some(args))),
            MessageBoxAction::SuccessDialog(message, task, args) =>
                *message_box.write_unchecked() = Some((MessageKind::Success, message, Some(task), Some(args))),
            MessageBoxAction::WarningDialog(message, task, args) =>
                *message_box.write_unchecked() = Some((MessageKind::Warning, message, Some(task), Some(args))),
        }
    }
}