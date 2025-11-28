use super::*;

#[component]
pub fn GatePassRenewButton(gate_pass_renew_dialog_visible_signal: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "hover:btn-neutral join-item",
            onclick: move |event| {
                event.prevent_default();
                event.stop_propagation();

                gate_pass_renew_dialog_visible_signal.set(true);
            },
            Icon { icon: Icons::RenewIcon, class: "size-8" }
            span {
                class: "opacity-0 group-hover:opacity-100",
                { t!("action-renew") }
            }
        }
    }
}

#[component]
pub fn GatePassRenewDialogView(
    gate_pass_selected_ids_signal: Signal<HashSet<String>>,
    gate_pass_renew_dialog_visible_signal: Signal<bool>,
) -> Element {
    if !gate_pass_renew_dialog_visible_signal() {
        return rsx! {};
    };
    let checked_gate_pass_renew = !gate_pass_selected_ids_signal.read().is_empty();
    let on_submit_gate_pass_reactivate = move |event: Event<FormData>| {
        event.prevent_default();
        event.stop_propagation();

        let ids = gate_pass_selected_ids_signal();
        let number_plates = event
            .get_str("number_plates")
            .filter(|string| !string.is_empty())
            .map(|string| split(string.as_ref(), ","));
        let payload = json!({
            "ids": ids,
            "number_plates": number_plates,
            "expired_at": event.get_str("expired_at"),
        });

        spawn(async move {
            if post_request!(url!(API_GATE_PASSES, "renews"), payload) {
                gate_pass_renew_dialog_visible_signal.set(false);
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
                                gate_pass_renew_dialog_visible_signal.set(false);
                            },
                            Icon { icon: Icons::Close, class: "size-4" }
                        }
                    }

                 form {
                    class: "flex grow flex-col items-center gap-3",
                    id: "gate-pass-reactivation-form",
                    autocomplete: "off",
                    onsubmit: on_submit_gate_pass_reactivate,
                    if checked_gate_pass_renew {
                        p { {format!("{} {}", t!("gate-pass-selected-renew-description"), gate_pass_selected_ids_signal.read().len())} }
                    } else {
                        p {
                            { t!("gate-pass-renew-description") }
                        }
                    }
                    FormDateField {
                        name: "expired_at",
                        title: "gate-pass-field-expired-at",
                        required: true,
                        initial_value: default_expired_at(),
                    }
                    if !checked_gate_pass_renew {
                        FormTextAreaField {
                            name: "number_plates",
                            title: "gate-pass-field-vehicle-number-plates",
                        }
                    }
                    div {style: "justify-content:center", class: "flex gap-3",
                        button {
                            class: "btn btn-primary",
                            { t!("gate-pass-action-renew-all") }
                        }
                    }
                }
            }
        }
    }
}
