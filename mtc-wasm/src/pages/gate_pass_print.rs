use super::*;

#[component]
pub fn GatePassPrintButton(
    gate_pass_id_print_signal: Signal<Option<String>>,
    gate_pass_print_visible_signal: Signal<bool>,
) -> Element {
    rsx! {
        button {
            class: "hover:btn-neutral join-item",
            onclick: move |event| {
                event.prevent_default();
                event.stop_propagation();
                gate_pass_id_print_signal.set(None);
                gate_pass_print_visible_signal.set(true);
            },
            Icon { icon: Icons::PrinterIcon, class: "size-8" }
            span {
                class: "opacity-0 group-hover:opacity-100",
                { t!("action-print") }
            }
        }
    }
}

#[component]
pub fn GatePassPrintDialogView(
    gate_pass_id_print_signal: Signal<Option<String>>,
    gate_pass_print_visible_signal: Signal<bool>,
) -> Element {
    if !gate_pass_print_visible_signal() {
        return rsx! {};
    };
    let two_side_print_modes = TwoSidePrintMode::values()
        .into_iter()
        .map(|mode| (format!("{:?}", mode), two_side_print_mode_name(&mode)))
        .collect::<Vec<(String, String)>>();
    let mut loading_spinner_hidden_signal = use_signal(|| true);
    let single_gate_pass_print = gate_pass_id_print_signal.read().is_some();
    let on_submit_gate_pass_print = move |event: Event<FormData>| {
        event.prevent_default();
        event.stop_propagation();

        let ids = gate_pass_id_print_signal().map(|gate_pass_id| vec![gate_pass_id]);
        let number_plates = event
            .get_str("number_plates")
            .filter(|string| !string.is_empty())
            .map(|string| split(string.as_ref(), ","));
        let payload = json!({
            "ids": ids,
            "number_plates": number_plates,
            "two_side_print_mode": event.get_str("two_side_print_mode"),
        });

        loading_spinner_hidden_signal.set(false);
        spawn(async move {
            match generate_print_gate_pass_html(payload).await {
                Ok(html) => {
                    jsFfiOpenHtml(html.as_str());
                    gate_pass_id_print_signal.set(None);
                    gate_pass_print_visible_signal.set(false);
                    loading_spinner_hidden_signal.set(true);
                }
                Err(error) => {
                    loading_spinner_hidden_signal.set(true);
                    error!("failed to generate Gate Pass print HTML: {error:?}");
                }
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
                                gate_pass_id_print_signal.set(None);
                                gate_pass_print_visible_signal.set(false);
                            },
                            Icon { icon: Icons::Close, class: "size-4" }
                        }
                    }

                 form {
                    class: "flex grow flex-col items-center gap-3",
                    id: "gate-pass-print-form",
                    autocomplete: "off",
                    onsubmit: on_submit_gate_pass_print,
                    if !single_gate_pass_print {
                        p {
                            { t!("gate-pass-print-description") }
                        }
                        FormTextAreaField {
                            name: "number_plates",
                            title: "gate-pass-field-vehicle-number-plates",
                        }
                    }
                    FormSimpleSelectField {
                        name: "two_side_print_mode",
                        title: "two-side-print-mode",
                        required: true,
                        selected: two_side_print_mode_name(&TwoSidePrintMode::default()),
                        items: two_side_print_modes,
                    }
                    div {style: "justify-content:center", class: "flex gap-3",
                        button {
                            class: "btn btn-primary",
                            span {
                                class: "loading loading-spinner",
                                hidden: loading_spinner_hidden_signal(),
                            }
                            if single_gate_pass_print {
                                { t!("gate-pass-action-print") }
                            } else {
                                { t!("gate-pass-action-print-all") }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn generate_print_gate_pass_html(print_gate_pass_requests: Value) -> Result<String, Error> {
    let bytes = state!(client)
        .post(url!(API_GATE_PASSES, "prints"))
        .json(&print_gate_pass_requests)
        .send()
        .await
        .map_err(|error| Error::Generic(Cow::Owned(error.to_string())))?
        .bytes()
        .await
        .map_err(|error| Error::Generic(Cow::Owned(error.to_string())))?;
    String::from_utf8(bytes.to_vec()).map_err(|error| Error::Generic(Cow::Owned(error.to_string())))
}

pub fn two_side_print_mode_name(mode: &TwoSidePrintMode) -> String {
    let key = format!("two-side-print-mode-{:?}", mode).to_lowercase();
    t!(key.as_str())
}
