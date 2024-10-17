#[macro_export]
macro_rules! close_dialog {
    () => {
        *use_dialog_box().write() = None
    };
}

#[macro_export]
macro_rules! alert_dialog {
    ($message:expr) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Success,
                message: t!($message),
                handler: None,
        })
    };

    ($message:expr, $handler:ident) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Alert,
                message: t!($message),
                handler: Some($handler),
        })
    };
}

#[macro_export]
macro_rules! success_dialog {
    ($message:expr) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Success,
                message: t!($message),
                handler: None,
        })
    };

    ($message:expr, $handler:ident) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Success,
                message: t!($message),
                handler: Some($handler),
        })
    };
}

#[macro_export]
macro_rules! error_dialog {
    ($message:expr) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Error,
                message: t!($message),
                handler: None,
        })
    };

    ($message:expr, $handler:ident) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Error,
                message: t!($message),
                handler: Some($handler),
        })
    };
}

#[macro_export]
macro_rules! info_dialog {
    ($message:expr) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Info,
                message: t!($message),
                handler: None,
        })
    };

    ($message:expr, $handler:ident) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Info,
                message: t!($message),
                handler: Some($handler),
        })
    };
}

#[macro_export]
macro_rules! warning_dialog {
    ($message:expr) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Warning,
                message: t!($message),
                handler: None,
        })
    };

    ($message:expr, $handler:ident) => {
        *use_dialog_box().write() =
            Some(DialogBoxArgs {
                kind: MessageKind::Warning,
                message: t!($message),
                handler: Some($handler),
        })
    };
}