use super::*;
use crate::pages::gate_pass_email::GatePassSendEmailDialogView;
use dioxus::prelude::*;
use std::collections::HashMap;
use tokio::join;

#[component]
pub fn GatePasses() -> Element {
    breadcrumbs!("menu-gate-passes");
    check_permission!(PERMISSION_GATE_PASS_READ);

    let search_signal = use_signal(String::new);
    let gate_pass_id_block_signal = use_signal(|| None::<String>);
    let gate_pass_block_signal = use_signal(|| None::<GatePassBlock>);
    let gate_pass_id_send_email_signal = use_signal(|| None::<String>);
    let gate_passes_resource = use_resource(move || async move {
        let search = search_signal();
        let upper_case_search = search.to_uppercase();
        let result: Vec<Value> = if search.is_empty() {
            let search_top_gate_passes = search_gate_passes(None, None).await.get_value().await;
            vec![search_top_gate_passes]
        } else {
            let gate_passes = join!(
                search_gate_passes(Some("last_name"), Some(&search)),
                search_gate_passes(Some("number_plate"), Some(&upper_case_search))
            );
            vec![
                gate_passes.0.get_value().await,
                gate_passes.1.get_value().await,
            ]
        };
        return result
            .into_iter()
            .flat_map(|value| {
                value
                    .self_obj::<Vec<GatePass>>()
                    .unwrap_or_default()
                    .into_iter()
            })
            .collect::<Vec<GatePass>>();
    });

    rsx! {
        section { class: "w-full grow xl:pr-16",
            GatePassSearchView { search_signal }
            GatePassBlockDialogView { gate_pass_id_block_signal, gate_pass_block_signal }
            GatePassSendEmailDialogView { gate_pass_id_send_email_signal }
            GatePassListView {
                gate_passes_resource,
                gate_pass_id_block_signal,
                gate_pass_id_send_email_signal,
                gate_pass_block_signal,
            }
        }
    }
}

fn search_gate_passes(
    criteria_name: Option<&str>,
    criteria_value: Option<&str>,
) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let mut payload = HashMap::new();
    if let (Some(name), Some(value)) = (criteria_name, criteria_value) {
        payload.insert(name.to_string(), value.to_string());
    }
    state!(client)
        .post(url!(API_GATE_PASSES, "searches"))
        .json(&payload)
        .send()
}

#[component]
pub fn GatePassSearchView(search_signal: Signal<String>) -> Element {
    rsx! {
        form {
            class: "w-full mb-6 pr-16 xl:pr-0",
            autocomplete: "off",
            onsubmit: move |event| {
                event.prevent_default();
                event.stop_propagation();
                search_signal.set(event.get_str("number_plate").unwrap_or_default().to_string());
            },
            label { class: "input input-sm flex grow mx-2 sm:mx-4 items-center gap-2",
                input {
                    class: "grow peer",
                    title: t!("gate-pass-action-search"),
                    style: "max-width: inherit; width: 100%",
                    r#type: "search",
                    name: "number_plate",
                    placeholder: &*t!("message-search"),
                }
                div { class: "relative -right-3 join",
                    button { class: "btn btn-sm btn-ghost join-item",
                        Icon { icon: Icons::Search, class: "size-6 text-primary" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GatePassListView(
    gate_passes_resource: Resource<Vec<GatePass>>,
    gate_pass_id_block_signal: Signal<Option<String>>,
    gate_pass_id_send_email_signal: Signal<Option<String>>,
    gate_pass_block_signal: Signal<Option<GatePassBlock>>,
) -> Element {
    let gate_passes = gate_passes_resource.suspend()?;

    rsx! {
        section { class: "w-full grow xl:pr-16",
            table { class: "entry-table",
                thead {
                    tr {
                        th { class: "w-12" }
                        th { class: "text-wrap", {t!("gate-pass-field-owner-full-name")} }
                        th { class: "text-wrap", {t!("gate-pass-field-vehicle-number-plate")} }
                        th { class: "text-wrap", {t!("gate-pass-field-vehicle-info")} }
                        th { class: "text-wrap", {t!("gate-pass-field-actions")} }
                    }
                }

                tbody {
                    for gate_pass in gate_passes().iter() {
                        {
                            let owner = &gate_pass.owner;
                            let vehicle = gate_pass.require_first_vehicle();
                            let block = gate_pass.block.clone();
                            let front_url = url!(API_GATE_PASSES, & gate_pass.id.as_ref(), "fronts");
                            let back_url = url!(API_GATE_PASSES, & gate_pass.id.as_ref(), "backs");
                            rsx! {
                                tr {
                                    onclick: {
                                        let gate_pass_id = gate_pass.id.to_string();
                                        move |_| {
                                            navigator().push(route!(API_ADMINISTRATOR, API_GATE_PASSES, gate_pass_id));
                                        }
                                    },
                                    td {
                                        a {
                                            class: "btn btn-xs btn-ghost",
                                            title: t!("gate-pass-action-print-front"),
                                            onclick: move |event| {
                                                event.prevent_default();
                                                event.stop_propagation();

                                                jsFfiOpenLink(&front_url);
                                            },
                                            Icon { icon: Icons::FrontSide, class: "size-6" }
                                        }
                                        a {
                                            class: "btn btn-xs btn-ghost",
                                            title: t!("gate-pass-action-print-back"),
                                            onclick: move |event| {
                                                event.prevent_default();
                                                event.stop_propagation();

                                                jsFfiOpenLink(&back_url);
                                            },
                                            Icon { icon: Icons::BackSide, class: "size-6" }
                                        }
                                    }
                                    td { {format!("{} {} {}", &*owner.last_name, &*owner.first_name, &*owner.middle_name)} }
                                    td { {vehicle.number_plate.as_ref()} }
                                    td {
                                        {
                                            format!(
                                                "{} {} {} {}",
                                                gate_pass_vehicle_color_name(&vehicle.color),
                                                gate_pass_vehicle_body_type_name(&vehicle.body_type),
                                                vehicle.manufacturer,
                                                vehicle.model.clone().unwrap_or_default(),
                                            )
                                        }
                                    }
                                    td {
                                        button {
                                            class: "btn btn-xl btn-ghost",
                                            title: if gate_pass.block.is_some() {
                                                {t!("gate-pass-action-edit-block")}
                                            } else {
                                                {t!("gate-pass-action-block")}
                                            },
                                            onclick: {
                                                let gate_pass_id = gate_pass.id.to_string();
                                                move |event| {
                                                    event.prevent_default();
                                                    event.stop_propagation();

                                                    gate_pass_id_block_signal.set(Some(gate_pass_id.clone()));
                                                    gate_pass_block_signal.set(block.clone());
                                                }
                                            },
                                            if gate_pass.blocked() {
                                                Icon { icon: Icons::ActiveBlockIcon, class: "size-8" }
                                            } else if gate_pass.block.is_some() {
                                                Icon { icon: Icons::ExpiredBlockIcon, class: "size-8" }
                                            } else {
                                                Icon { icon: Icons::InactiveBlockIcon, class: "size-8" }
                                            }
                                        }

                                        button {
                                            class: "btn btn-xl btn-ghost",
                                            title: t!("gate-pass-action-send-email"),
                                            onclick: {
                                                let gate_pass_id = gate_pass.id.to_string();
                                                move |event| {
                                                    event.prevent_default();
                                                    event.stop_propagation();

                                                    gate_pass_id_send_email_signal.set(Some(gate_pass_id.clone()));
                                                }
                                            },
                                            Icon { icon: Icons::SendEmail, class: "size-8" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            EntriesActions {
                future: gate_passes_resource,
                route: route!(API_ADMINISTRATOR, API_GATE_PASSES, ID_CREATE),
                permission: PERMISSION_GATE_PASS_WRITE,
                extra_buttons: vec!(GatePassExportButton(), GatePassImportButton())
            }
        }
    }
}

pub fn gate_pass_owner_title_name(title: &GatePassOwnerTitle) -> String {
    let key = format!("gate-pass-owner-title-{:?}", title).to_lowercase();
    t!(key.as_str())
}

pub fn gate_pass_vehicle_color_name(color: &VehicleColor) -> String {
    let key = format!("gate-pass-vehicle-color-{:?}", color).to_lowercase();
    t!(key.as_str())
}

pub fn gate_pass_vehicle_body_type_name(body_type: &VehicleBodyType) -> String {
    let key = format!("gate-pass-vehicle-body-type-{:?}", body_type).to_lowercase();
    t!(key.as_str())
}

pub fn gate_pass_allow_any_vehicle_name(allow_any_vehicle: &bool) -> String {
    if allow_any_vehicle == &true {
        t!("field-yes")
    } else {
        t!("field-no")
    }
}
