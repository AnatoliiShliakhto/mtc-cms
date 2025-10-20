use super::*;
use dioxus::prelude::*;

#[component]
pub fn GatePassSendEmailDialogView(
    gate_pass_id_send_email_signal: Signal<Option<String>>,
) -> Element {
    if gate_pass_id_send_email_signal.read().is_none() {
        return rsx! {};
    };

    let on_submit_gate_pass_send_email = move |event: Event<FormData>| {
        event.prevent_default();
        event.stop_propagation();

        let payload = json!({
            "recipient_email": event.get_str("recipient_email"),
        });

        spawn(async move {
            let request_result = post_request!(
                url!(
                    API_GATE_PASSES,
                    gate_pass_id_send_email_signal.read().as_ref().unwrap(),
                    "emails"
                ),
                payload
            );
            if request_result {
                gate_pass_id_send_email_signal.set(None);
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
                                gate_pass_id_send_email_signal.set(None);
                            },
                            Icon { icon: Icons::Close, class: "size-4" }
                        }
                    }

                 form {
                    class: "flex grow flex-col items-center gap-3",
                    id: "gate-pass-send-email-form",
                    autocomplete: "off",
                    onsubmit: on_submit_gate_pass_send_email,
                    FormTextField {
                        r#type: "email",
                        name: "recipient_email",
                        title: "gate-pass-field-recipient-email",
                        required: true,
                    }
                    button {
                        class: "btn btn-primary",
                        onclick: |_| {
                        },
                        { t!("gate-pass-action-send-email") }
                    }
                }
            }
        }
    }
}