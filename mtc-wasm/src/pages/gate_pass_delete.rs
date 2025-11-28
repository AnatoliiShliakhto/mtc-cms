use super::*;

#[component]
pub fn GatePassDeleteButton(gate_pass_delete_dialog_visible_signal: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "hover:btn-neutral join-item",
            onclick: move |event| {
                event.prevent_default();
                event.stop_propagation();

                gate_pass_delete_dialog_visible_signal.set(true);
            },
            Icon { icon: Icons::Trash, class: "size-8" }
            span { class: "opacity-0 group-hover:opacity-100", {t!("action-delete")} }
        }
    }
}

#[component]
pub fn GatePassDeleteDialogView(
    gate_pass_selected_ids_signal: Signal<HashSet<String>>,
    gate_pass_delete_dialog_visible_signal: Signal<bool>,
) -> Element {
    if !gate_pass_delete_dialog_visible_signal() {
        return rsx! {};
    };
    let mut loading_spinner_hidden_signal = use_signal(|| true);
    let on_submit_gate_pass_delete = move |event: Event<FormData>| {
        event.prevent_default();
        event.stop_propagation();

        loading_spinner_hidden_signal.set(false);
        spawn(async move {
            let ids = gate_pass_selected_ids_signal();
            for id in ids {
                if !delete_request!(url!(API_GATE_PASSES, &id)) {
                    error!("failed to delete gate pass: gate_pass_id={id}",)
                }
            }
            loading_spinner_hidden_signal.set(true);
            gate_pass_delete_dialog_visible_signal.set(false);
            navigator().push(route!(API_ADMINISTRATOR, "loops"));
        });
    };

    rsx! {
        section { class: "modal modal-open",
            div { class: "modal-box",
                div { class: "absolute top-0 right-0 join rounded-none",
                    button {
                        class: "btn btn-sm btn-ghost join-item hover:text-error",
                        onclick: move |_| {
                            gate_pass_delete_dialog_visible_signal.set(false);
                        },
                        Icon { icon: Icons::Close, class: "size-4" }
                    }
                }

                form {
                    class: "flex grow flex-col items-center gap-3",
                    id: "gate-pass-delete-form",
                    autocomplete: "off",
                    onsubmit: on_submit_gate_pass_delete,
                    p { {format!("{} {}", t!("gate-pass-selected-delete-description"), gate_pass_selected_ids_signal.read().len())} }
                    div { style: "justify-content:center", class: "flex gap-3",
                        button { class: "btn btn-primary",
                            span {
                                class: "loading loading-spinner",
                                hidden: loading_spinner_hidden_signal(),
                            }
                            {t!("gate-pass-action-delete-all")}
                        }
                    }
                }
            }
        }
    }
}
