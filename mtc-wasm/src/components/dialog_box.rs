use super::*;

/// A component to show a dialog box.
///
/// The component shows a modal box with a message and an optional action. The
/// message is given by the `message` field of the `DialogBoxArgs` struct. The
/// action is given by the `handler` field of the same struct. If `handler` is
/// `None`, the component shows a single button with the label "Close". If
/// `handler` is `Some`, the component shows two buttons with the labels "Yes"
/// and "No". Clicking "Yes" triggers the handler and closes the dialog. Clicking
/// "No" just closes the dialog.
///
/// The component uses the `dialog` state to get the message and the action. If
/// the `dialog` state is `None`, the component returns an empty element.
///
/// The component styles the dialog box according to the kind of the message. If
/// the kind is `Alert`, the dialog box is styled with the `alert` class. If the
/// kind is `Info`, the dialog box is styled with the `info` class. If the kind is
/// `Error`, the dialog box is styled with the `error` class. If the kind is
/// `Success`, the dialog box is styled with the `success` class. If the kind is
/// `Warning`, the dialog box is styled with the `warning` class.
#[component]
pub fn DialogBox() -> Element {
    let Some(args) = state!(dialog) else {
        return rsx!{}
    };

    rsx! {
        section {
            class: "modal modal-open",
            onclick: move |_| close_dialog!(),
            div {
                class: "modal-box",
                onclick: |event| event.stop_propagation(),
                div {
                    class: "absolute top-0 right-0 join rounded-none",
                    button {
                        class: "btn btn-sm btn-ghost join-item hover:text-error",
                        onclick: move |_| close_dialog!(),
                        Icon { icon: Icons::Close, class: "size-4" }
                    }
                }

                match args.kind {
                    MessageKind::Alert => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold wrap",
                            Icon { icon: Icons::Alert, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { t!("caption-alert") }
                                div { class: "divider my-0" }
                            }
                        }
                    },
                    MessageKind::Info => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold wrap text-info",
                            Icon { icon: Icons::Info, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { t!("caption-info") }
                                div { class: "divider my-0" }
                            }
                        }
                    },
                    MessageKind::Error => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold wrap text-error",
                            Icon { icon: Icons::Error, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { t!("caption-error") }
                                div { class: "divider my-0" }
                            }
                        }
                    },
                    MessageKind::Success => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold wrap text-success",
                            Icon { icon: Icons::Success, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { t!("caption-success") }
                                div { class: "divider my-0" }
                            }
                        }
                    },
                    MessageKind::Warning => rsx! {
                        div {
                            class: "flex flex-row gap-4 text-lg font-bold wrap text-warning",
                            Icon { icon: Icons::Warning, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { t!("caption-warning") }
                                div { class: "divider my-0" }
                            }
                        }
                    }
                }

                p {
                    class: "indent-14",
                    { args.message }
                }
                div {
                    class: "card-actions mt-6 gap-6 justify-end",
                    if let Some(handler) = args.handler {
                        button {
                            class: match args.kind {
                                MessageKind::Alert => "btn btn-primary",
                                MessageKind::Info => "btn btn-info",
                                MessageKind::Error => "btn btn-error",
                                MessageKind::Success => "btn btn-success",
                                MessageKind::Warning => "btn btn-warning",
                            },
                            onclick: move |event| {
                                close_dialog!();
                                handler(event)
                            },
                            { t!("action-yes") }
                        }
                        button {
                            class: "btn btn-outline",
                            onclick: move |_| close_dialog!(),
                            { t!("action-no") }
                        }
                    } else {
                        button {
                            class: "btn btn-primary",
                            onclick: move |_| close_dialog!(),
                            { t!("action-close") }
                        }
                    }
                }
            }
        }
    }
}