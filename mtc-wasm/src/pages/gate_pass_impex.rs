use super::*;
use crate::prelude::dioxus_elements::FileEngine;
use csv::{Reader, Writer};
use futures_util::TryFutureExt;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;
use validator::Validate;
use wasm_bindgen_futures::JsFuture;

//
// Import
//
#[component]
pub fn GatePassImportButton() -> Element {
    let gate_pass_import_element_id = "gate-pass-import";
    let import_from_csv = move |event: Event<FormData>| async move {
        event.prevent_default();
        event.stop_propagation();

        match read_from_csv_file(event.files())
            .and_then(create_gate_passes)
            .await
        {
            Ok(response) => {
                if !response.failed_requests.is_empty() {
                    let number_of_succeeded = response.created_gate_passes.len();
                    let number_of_failed = response.failed_requests.len();
                    let export_to_csv = Callback::new(move |event: MouseEvent| {
                        let response = response.clone();
                        spawn(async move {
                            if let Err(error) =
                                write_failed_create_gate_pass_requests_to_csv(response).await
                            {
                                error!("failed export to csv {:?}", error);
                            };
                            navigator().push(route!(API_ADMINISTRATOR, "loops"));
                        });
                    });
                    let message = t!("gate-pass-import-error")
                        .replace("{succeeded}", &number_of_succeeded.to_string())
                        .replace("{failed}", &number_of_failed.to_string());
                    alert_dialog!(message.as_str(), export_to_csv);
                } else {
                    navigator().push(route!(API_ADMINISTRATOR, "loops"));
                }
            }
            Err(error) => {
                let error = format!("{}: {}", t!("error-import"), error.message());
                error_dialog!(error.as_str());
            }
        };
    };

    rsx! {
        input {
            class: "hidden",
            id: gate_pass_import_element_id,
            r#type: "file",
            accept: ".csv",
            multiple: false,
            onchange: import_from_csv
        }
        button {
            class: "hover:btn-neutral join-item",
            onclick: |event| {
                event.prevent_default();
                event.stop_propagation();
                jsFfiClickElement(gate_pass_import_element_id);
            },
            Icon { icon: Icons::Upload, class: "size-8" }
            span {
                class: "opacity-0 group-hover:opacity-100",
                { t!("action-upload") }
            }
        }
    }
}

struct ImportMappers {
    title_name_to_title: HashMap<String, GatePassOwnerTitle>,
    color_name_to_color: HashMap<String, VehicleColor>,
    body_type_name_to_body_type: HashMap<String, VehicleBodyType>,
    allow_any_vehicle_name_to_allow_any_vehicle: HashMap<String, bool>,
}

impl ImportMappers {
    fn new() -> Self {
        ImportMappers {
            title_name_to_title: GatePassOwnerTitle::values()
                .into_iter()
                .map(|value| (gate_pass_owner_title_name(&value).to_lowercase(), value))
                .collect::<HashMap<_, _>>(),
            color_name_to_color: VehicleColor::values()
                .into_iter()
                .map(|value| (gate_pass_vehicle_color_name(&value).to_lowercase(), value))
                .collect::<HashMap<_, _>>(),
            body_type_name_to_body_type: VehicleBodyType::values()
                .into_iter()
                .map(|value| {
                    (
                        gate_pass_vehicle_body_type_name(&value).to_lowercase(),
                        value,
                    )
                })
                .collect::<HashMap<_, _>>(),
            allow_any_vehicle_name_to_allow_any_vehicle: vec![true, false]
                .into_iter()
                .map(|value| {
                    let allow_any_vehicle = if value {
                        t!("field-yes").to_lowercase()
                    } else {
                        t!("field-no").to_lowercase()
                    };
                    (allow_any_vehicle, value)
                })
                .collect::<HashMap<_, _>>(),
        }
    }

    fn to_owner_title(&self, owner_title_name: &str) -> Option<&GatePassOwnerTitle> {
        self.title_name_to_title
            .get(&owner_title_name.trim().to_lowercase())
    }

    fn to_vehicle_color(&self, vehicle_color_name: &str) -> Option<&VehicleColor> {
        self.color_name_to_color
            .get(&vehicle_color_name.trim().to_lowercase())
    }

    fn to_vehicle_allow_any_vehicle(&self, allow_any_vehicle_name: &str) -> Option<&bool> {
        self.allow_any_vehicle_name_to_allow_any_vehicle
            .get(&allow_any_vehicle_name.trim().to_lowercase())
    }

    fn to_vehicle_body_type(&self, vehicle_body_type_name: &str) -> Option<&VehicleBodyType> {
        self.body_type_name_to_body_type
            .get(&vehicle_body_type_name.trim().to_lowercase())
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct CreateGatePassRequestCsvRecord {
    #[validate(length(min = 1))]
    pub expired_at: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub last_name: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub first_name: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub middle_name: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub unit: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub title: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub allow_any_vehicle: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub number_plate: Cow<'static, str>,
    pub vin_code: Option<Cow<'static, str>>,
    #[validate(length(min = 1))]
    pub manufacturer: Cow<'static, str>,
    pub model: Option<Cow<'static, str>>,
    #[validate(length(min = 1))]
    pub color: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub body_type: Cow<'static, str>,
}

async fn read_from_csv_file(
    file_engine_opt: Option<Arc<dyn FileEngine>>,
) -> Result<Vec<CreateGatePassRequest>, Error> {
    if let Some(file_engine) = file_engine_opt {
        if let Some(file_name) = file_engine.files().into_iter().next() {
            if let Some(csv_bytes) = file_engine.read_file(&file_name).await {
                let cursor = Cursor::new(csv_bytes);
                let mut csv_reader = Reader::from_reader(cursor);
                let import_mappers = ImportMappers::new();
                return csv_reader
                    .deserialize::<CreateGatePassRequestCsvRecord>()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|record_result| {
                        record_result
                            .map_err(|error| Error::Generic(Cow::Owned(error.to_string())))
                            .and_then(validate_create_gate_pass_csv_request_record)
                            .and_then(|record| {
                                convert_to_create_gate_pass_request(&import_mappers, record)
                            })
                    })
                    .collect();
            }
        }
    }
    Ok(Vec::new())
}

fn validate_create_gate_pass_csv_request_record(
    record: CreateGatePassRequestCsvRecord,
) -> Result<CreateGatePassRequestCsvRecord, Error> {
    record
        .validate()
        .map_err(|error| {
            Error::Generic(Cow::Owned(format!(
                "error={:?}, record={:?}",
                error, record
            )))
        })
        .map(|()| record)
}

fn convert_to_create_gate_pass_request(
    mappers: &ImportMappers,
    record: CreateGatePassRequestCsvRecord,
) -> Result<CreateGatePassRequest, Error> {
    let title = mappers.to_owner_title(&record.title);
    let color = mappers.to_vehicle_color(&record.color);
    let body_type = mappers.to_vehicle_body_type(&record.body_type);
    let allow_any_vehicle = mappers.to_vehicle_allow_any_vehicle(&record.allow_any_vehicle);

    // unknown enum values
    if title.is_none() || color.is_none() || body_type.is_none() || allow_any_vehicle.is_none() {
        return Err(Error::Generic(Cow::Owned(format!("record={:?}", record))));
    }

    let request = CreateGatePassRequest {
        expired_at: record.expired_at.clone(),
        owner: GatePassOwner {
            last_name: record.last_name.clone(),
            first_name: record.first_name.clone(),
            middle_name: record.middle_name.clone(),
            title: title.unwrap().clone(),
            unit: record.unit.clone(),
        },
        vehicles: vec![GatePassVehicle {
            number_plate: record.number_plate.clone(),
            vin_code: record.vin_code.clone(),
            manufacturer: record.manufacturer.clone(),
            model: record.model.clone(),
            color: color.unwrap().clone(),
            body_type: body_type.unwrap().clone(),
        }],
        allow_any_vehicle: allow_any_vehicle.unwrap().clone(),
        created_by: None,
        updated_by: None,
    };

    Ok(request)
}

fn convert_to_create_gate_pass_csv_request_record(
    create_gate_pass_request: &CreateGatePassRequest,
) -> CreateGatePassRequestCsvRecord {
    let owner = &create_gate_pass_request.owner;
    let vehicle = &create_gate_pass_request.vehicles.first().unwrap();
    CreateGatePassRequestCsvRecord {
        expired_at: create_gate_pass_request.expired_at.clone(),
        last_name: owner.last_name.clone(),
        first_name: owner.first_name.clone(),
        middle_name: owner.middle_name.clone(),
        title: Cow::Owned(gate_pass_owner_title_name(&owner.title)),
        unit: owner.unit.clone(),
        number_plate: vehicle.number_plate.clone(),
        manufacturer: vehicle.manufacturer.clone(),
        model: vehicle.model.clone(),
        color: Cow::Owned(gate_pass_vehicle_color_name(&vehicle.color)),
        allow_any_vehicle: Cow::Owned(gate_pass_allow_any_vehicle_name(
            &create_gate_pass_request.allow_any_vehicle,
        )),
        body_type: Cow::Owned(gate_pass_vehicle_body_type_name(&vehicle.body_type)),
        vin_code: vehicle.vin_code.clone(),
    }
}

async fn create_gate_passes(
    create_gate_pass_requests: Vec<CreateGatePassRequest>,
) -> Result<CreateGatePassBatchResponse, Error> {
    let request = CreateGatePassBatchRequest {
        requests: create_gate_pass_requests,
    };
    state!(client)
        .post(url!(API_GATE_PASSES, "imports"))
        .json(&request)
        .send()
        .await
        .map_err(|error| {
            error!("{:?}", error);
            Error::Generic(Cow::Owned(t!("error-recommendation").to_lowercase()))
        })?
        .json::<CreateGatePassBatchResponse>()
        .await
        .map_err(|error| {
            error!("{:?}", error);
            Error::Generic(Cow::Owned(t!("error-recommendation").to_lowercase()))
        })
}

async fn write_failed_create_gate_pass_requests_to_csv(
    response: CreateGatePassBatchResponse,
) -> Result<(), Error> {
    if !response.failed_requests.is_empty() {
        return write_to_csv_file(
            response.failed_requests,
            convert_to_create_gate_pass_csv_request_record,
            "import_gate_pass_errors.csv",
        )
        .await;
    }
    Ok(())
}

//
// Export
//
#[component]
pub fn GatePassExportButton() -> Element {
    let export_to_csv = move |event: Event<MouseData>| async move {
        event.prevent_default();
        event.stop_propagation();

        match get_gate_passes()
            .and_then(|gate_passes| {
                write_to_csv_file(
                    gate_passes,
                    convert_to_gate_pass_csv_record,
                    "gate_passes.csv",
                )
            })
            .await
        {
            Ok(_) => (),
            Err(error) => {
                let error = format!("{}: {}", t!("error-export"), error.message());
                error_dialog!(error.as_str());
            }
        };
    };

    rsx! {
        button {
            class: "hover:btn-neutral join-item",
            onclick: export_to_csv,
            Icon { icon: Icons::Download, class: "size-8" }
            span {
                class: "opacity-0 group-hover:opacity-100",
                { t!("action-export") }
            }
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct GatePassCsvRecord {
    #[validate(length(min = 1))]
    pub id: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub number: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub expired_at: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub last_name: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub first_name: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub middle_name: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub unit: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub title: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub allow_any_vehicle: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub number_plate: Cow<'static, str>,
    pub vin_code: Option<Cow<'static, str>>,
    #[validate(length(min = 1))]
    pub manufacturer: Cow<'static, str>,
    pub model: Option<Cow<'static, str>>,
    #[validate(length(min = 1))]
    pub color: Cow<'static, str>,
    #[validate(length(min = 1))]
    pub body_type: Cow<'static, str>,
}

async fn get_gate_passes() -> Result<Vec<GatePass>, Error> {
    state!(client)
        .post(url!(API_GATE_PASSES, "exports"))
        .send()
        .await
        .map_err(|error| Error::Generic(Cow::Owned(error.to_string())))?
        .json::<Vec<GatePass>>()
        .await
        .map_err(|error| Error::Generic(Cow::Owned(error.to_string())))
}

async fn write_to_csv_file<F, S, T>(
    entities: Vec<S>,
    to_csv_record: F,
    file_name: &str,
) -> Result<(), Error>
where
    F: Fn(&S) -> T,
    S: Serialize,
    T: Serialize,
{
    let csv_string = to_csv_string(entities, to_csv_record)?;
    if JsFuture::from(jsFfiExportCsvFile(&csv_string, file_name))
        .await
        .is_err()
    {
        error!("failed to invoke jsFfiExportCsvFile");
    }
    Ok(())
}

fn to_csv_string<F, S, T>(entities: Vec<S>, to_csv_record: F) -> Result<String, Error>
where
    F: Fn(&S) -> T,
    S: Serialize,
    T: Serialize,
{
    let mut writer = Writer::from_writer(vec![]);
    for entity in entities.iter() {
        writer.serialize(to_csv_record(entity)).map_err(|error| {
            error!("failed to serialize: error={:?}", error);
            Error::Generic(Cow::Owned(error.to_string()))
        })?;
    }
    let csv_bytes = writer.into_inner().map_err(|error| {
        error!("failed to serialize: error={:?}", error);
        Error::Generic(Cow::Owned(error.to_string()))
    })?;
    String::from_utf8(csv_bytes).map_err(|error| Error::Generic(Cow::Owned(error.to_string())))
}

fn convert_to_gate_pass_csv_record(gate_pass: &GatePass) -> GatePassCsvRecord {
    let owner = &gate_pass.owner;
    let vehicle = &gate_pass.require_first_vehicle();
    GatePassCsvRecord {
        id: gate_pass.id.clone(),
        number: gate_pass.number.clone(),
        expired_at: gate_pass.expired_at.clone(),
        last_name: owner.last_name.clone(),
        first_name: owner.first_name.clone(),
        middle_name: owner.middle_name.clone(),
        title: Cow::Owned(gate_pass_owner_title_name(&owner.title)),
        unit: owner.unit.clone(),
        number_plate: vehicle.number_plate.clone(),
        manufacturer: vehicle.manufacturer.clone(),
        model: vehicle.model.clone(),
        color: Cow::Owned(gate_pass_vehicle_color_name(&vehicle.color)),
        allow_any_vehicle: Cow::Owned(gate_pass_allow_any_vehicle_name(
            &gate_pass.allow_any_vehicle,
        )),
        body_type: Cow::Owned(gate_pass_vehicle_body_type_name(&vehicle.body_type)),
        vin_code: vehicle.vin_code.clone(),
    }
}
