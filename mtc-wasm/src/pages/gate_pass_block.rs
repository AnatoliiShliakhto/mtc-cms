use super::*;
use dioxus::prelude::*;

#[component]
pub fn GatePassBlockDialogView(
    gate_pass_id_block_signal: Signal<Option<String>>,
    gate_pass_block_signal: Signal<Option<GatePassBlock>>,
) -> Element {
    if gate_pass_id_block_signal.read().is_none() {
        return rsx! {};
    };

    let mut unblock_signal = use_signal(|| false);
    let on_submit_gate_pass_block = move |event: Event<FormData>| {
        event.prevent_default();
        event.stop_propagation();

        let payload = if unblock_signal() {
            json!({})
        } else {
            let block = json!({
                "expired_at": event.get_str("expired_at"),
                "reason": event.get_str("reason"),
            });
            json!({
                "block": block,
            })
        };

        spawn(async move {
            let request_result = post_request!(
                url!(
                    API_GATE_PASSES,
                    gate_pass_id_block_signal.read().as_ref().unwrap(),
                    "blocks"
                ),
                payload
            );
            if request_result {
                gate_pass_block_signal.set(None);
                gate_pass_id_block_signal.set(None);
                navigator().push(route!(API_ADMINISTRATOR, "loops"));
            }
        });
    };

    rsx! {
            section {
                class: "modal modal-open",
                div {
                    class: "modal-box",
                    div {
                        class: "absolute top-0 right-0 join rounded-none",
                        button {
                            class: "btn btn-sm btn-ghost join-item hover:text-error",
                            onclick: move |_| {
                                gate_pass_block_signal.set(None);
                                gate_pass_id_block_signal.set(None);
                            },
                            Icon { icon: Icons::Close, class: "size-4" }
                        }
                    }

                 form {
                    class: "flex grow flex-col items-center gap-3",
                    id: "gate-pass-block-form",
                    autocomplete: "off",
                    onsubmit: on_submit_gate_pass_block,
                    FormDateField {
                        name: "expired_at",
                        title: "gate-pass-field-block-expired-at",
                        required: true,
                        initial_value: gate_pass_block_signal.read().as_ref()
                            .map(|block| block.expired_at[0..10].to_string()),
                    }
                    FormTextAreaField {
                        name: "reason",
                        title: "gate-pass-field-block-reason",
                        required: true,
                        initial_value: gate_pass_block_signal.read().as_ref()
                            .map(|block| block.reason.to_string()),
                    }
                    div {style: "justify-content:center", class: "flex gap-3",
                        button {
                            class: "btn btn-primary",
                            disabled: gate_pass_block_signal.read().is_none(),
                            onclick: move |event| {
                                unblock_signal.set(true);
                            },
                            { t!("gate-pass-action-unblock") }
                        }
                        button {
                            class: "btn btn-primary",
                            if gate_pass_block_signal.read().is_some() {
                                { t!("gate-pass-action-reblock") }
                            } else {
                                { t!("gate-pass-action-block") }
                            }
                        }
                    }
                }
            }
        }
    }
}