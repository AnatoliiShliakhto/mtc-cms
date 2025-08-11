use super::*;
use std::collections::HashMap;
use tokio::join;

#[component]
pub fn GatePasses() -> Element {
    breadcrumbs!("menu-gate-passes");
    check_permission!(PERMISSION_GATE_PASS_READ);
    let mut search = use_signal(String::new);
    let gate_passes_resource = use_resource(move || async move {
        let search = search();
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
    let gate_passes = gate_passes_resource.suspend()?;

    rsx! {
        section { class: "w-full grow xl:pr-16",
            form {
                class: "w-full mb-6 pr-16 xl:pr-0",
                autocomplete: "off",
                onsubmit: move |event| {
                    event.prevent_default();
                    event.stop_propagation();
                    search.set(event.get_str("number_plate").unwrap_or_default().to_string());
                },
                label { class: "input input-sm flex grow mx-2 sm:mx-4 items-center gap-2",
                    input {
                        class: "grow peer",
                        style: "max-width: inherit; width: 100%",
                        r#type: "search",
                        name: "number_plate",
                        placeholder: &*t!("message-search"),
                    }
                    div { class: "relative -right-3 join",
                        button { class: "btn btn-sm btn-ghost join-item",
                            Icon {
                                icon: Icons::Search,
                                class: "size-6 text-primary",
                            }
                        }
                    }
                }
            }

            table { class: "entry-table",
                thead {
                    tr {
                        th { class: "w-8" }
                        th { class: "text-wrap", {t!("gate-pass-field-owner-full-name")} }
                        th { {t!("gate-pass-field-vehicle-number-plate")} }
                        th { {t!("gate-pass-field-vehicle-info")} }
                    }
                }

                tbody {
                    for gate_pass in gate_passes().iter() {
                        {
                            let gate_pass_id = gate_pass.id.to_string();
                            let owner = &gate_pass.owner;
                            let vehicle = &gate_pass.vehicle;
                            let qr_code_url = url!(API_GATE_PASSES, &gate_pass.id.as_ref(), "qr-codes");
                            rsx! {
                                tr {
                                    onclick: move |_| {
                                        navigator().push(route!(API_ADMINISTRATOR, API_GATE_PASSES, gate_pass_id));
                                    },
                                    td {
                                        a{
                                            class: "btn btn-xs btn-ghost",
                                            onclick: move |event| {
                                                event.prevent_default();
                                                event.stop_propagation();
                                                jsFfiOpenLink(&qr_code_url);
                                            },
                                            Icon { icon: Icons::QrCode, class: "size-4" }
                                        }
                                    }
                                    td { {format!("{} {} {}", &*owner.last_name, &*owner.first_name, &*owner.middle_name)} }
                                    td { {&*vehicle.number_plate} }
                                    td { {format!("{} {} {} {}", t!(format!("gate-pass-vehicle-color-{:?}", vehicle.color).to_lowercase().as_str()), t!(format!("gate-pass-vehicle-body-type-{:?}", vehicle.body_type).to_lowercase().as_str()), vehicle.manufacturer, vehicle.model.clone().unwrap_or_default())} }
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
            }
        }
    }
}

fn search_gate_passes(
    criteria_name: Option<&str>,
    criteria_value: Option<&str>,
) -> impl Future<Output = Result<Response, reqwest::Error>> {
    let mut payload = HashMap::new();
    if criteria_name.is_some() && criteria_value.is_some() {
        payload.insert(criteria_name.as_ref(), criteria_value.as_ref());
    }
    state!(client)
        .post(url!("gate-pass-searches"))
        .json(&payload)
        .send()
}
