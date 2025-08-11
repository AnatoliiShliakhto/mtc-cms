use super::*;
use std::collections::HashMap;
use tokio::{join, try_join};

/// Quiz list page provides brief quiz information and allows:
/// - create quiz
/// - assign quiz to user groups
/// - select quiz
#[component]
pub fn GatePasses() -> Element {
    breadcrumbs!("menu-gate-passes");
    let res = check_permission!(PERMISSION_GATE_PASS_READ);
    let mut search = use_signal(String::new);
    let future = use_resource(move || async move {
        let mut payload_search_by_number_plate = HashMap::new();
        payload_search_by_number_plate.insert("number_plate", search().to_uppercase());
        let search_by_number_plate = state!(client)
            .post(url!(API_GATE_PASS_SEARCHES))
            .json(&payload_search_by_number_plate)
            .send();
        let mut payload_search_by_last_name = HashMap::new();
        payload_search_by_last_name.insert("last_name", search());
        let search_by_last_name = state!(client)
            .post(url!(API_GATE_PASS_SEARCHES))
            .json(&payload_search_by_last_name)
            .send();
        let result = join!(
            search_by_number_plate.await.get_value(),
            search_by_last_name.await.get_value()
        );
        return vec![result.0, result.1]
            .into_iter()
            .flat_map(|value| {
                value
                    .self_obj::<Vec<GatePass>>()
                    .unwrap_or_default()
                    .into_iter()
            })
            .collect::<Vec<GatePass>>();
    });
    let response = future.suspend()?;
    let writer = state!(auth).has_permission(PERMISSION_GATE_PASS_WRITE);

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
                        th { class: "text-wrap", {t!("field-owner-full-name")} }
                        th { {t!("field-vehicle-number-plate")} }
                        th { {t!("field-vehicle-info")} }
                    }
                }

                tbody {
                    for gate_pass in response().iter() {
                        {
                            let gate_pass_id = gate_pass.id.to_string();
                            let owner = &gate_pass.owner;
                            let vehicle = &gate_pass.vehicle;
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
                                            },
                                            Icon { icon: Icons::QrCode, class: "size-4" }
                                        }
                                    }
                                    td { {format!("{} {} {}", &*owner.last_name, &*owner.first_name, &*owner.middle_name)} }
                                    td { {&*vehicle.number_plate} }
                                    td { {format!("{:?} {}", vehicle.color, vehicle.manufacturer)} }
                                }
                            }
                        }
                    }
                }
            }

            EntriesActions {
                future,
                route: route!(API_ADMINISTRATOR, API_GATE_PASSES, ID_CREATE),
                permission: PERMISSION_GATE_PASS_WRITE,
            }
        }
    }
}