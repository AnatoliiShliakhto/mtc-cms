use super::*;

#[component]
pub fn MessageBox(
    #[props]
    kind: MessageKind,
    #[props(into)]
    message: String,
    #[props]
    task: Option<MessageBoxFn>,
    #[props]
    task_args: Option<MessageBoxFnArgs>,
) -> Element {

    rsx! {
        section {
            class: "modal modal-open",
            onclick: close_message_box_task,
            div {
                class: "modal-box",
                onclick: |event| event.stop_propagation(),
                div {
                    class: "absolute top-0 right-0 join rounded-none",
                    button {
                        class: "btn btn-sm btn-ghost join-item hover:text-error",
                        onclick: close_message_box_task,
                        Icon { icon: Icons::Close, class: "size-4" }
                    }
                }

                match kind {
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
                    { message } 
                }
                div { 
                    class: "card-actions mt-6 gap-6 justify-end",
                    if let Some(task) = task {
                        button {
                            class: match kind {
                                MessageKind::Alert => "btn btn-primary",
                                MessageKind::Info => "btn btn-info",
                                MessageKind::Error => "btn btn-error",
                                MessageKind::Success => "btn btn-success",
                                MessageKind::Warning => "btn btn-warning",
                            },
                            onclick: move |_| {
                                let args = task_args.clone().unwrap_or_default();
                                task(args.0, args.1)
                            },
                            { t!("action-yes") }
                        }
                        button {
                            class: "btn btn-outline",
                            onclick: close_message_box_task,
                            { t!("action-no") }
                        }
                    } else {
                        button {
                            class: "btn btn-primary",
                            onclick: close_message_box_task,
                            { t!("action-close") }
                        }
                    }    
                }
            }
        }    
    }
}