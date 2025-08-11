use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GatePass {
    pub id: Cow<'static, str>,
    pub expired_at: Cow<'static, str>,
    pub deleted: bool,
    pub owner: GatePassOwner,
    pub vehicle: GatePassVehicle,
    pub allow_any_vehicle: bool,
    pub created_at: Cow<'static, str>,
    pub updated_at: Cow<'static, str>,
    pub created_by: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateGatePassRequest {
    pub expired_at: Cow<'static, str>,
    pub owner: GatePassOwner,
    pub vehicle: GatePassVehicle,
    pub allow_any_vehicle: bool,
    pub created_by: Option<Cow<'static, str>>,
    pub updated_by: Option<Cow<'static, str>>,
}

impl CreateGatePassRequest {
    pub fn normalize(&mut self) {
        self.owner.last_name = normalize(&self.owner.last_name);
        self.owner.first_name = normalize(&self.owner.first_name);
        self.owner.middle_name = normalize(&self.owner.middle_name);
        self.owner.title = normalize(&self.owner.title);
        self.owner.unit = uppercase(&self.owner.unit);
        self.vehicle.number_plate = uppercase(&self.vehicle.number_plate);
        self.vehicle.vin_code = self
            .vehicle
            .vin_code
            .as_ref()
            .map(|vin_code| uppercase(&vin_code));
        self.vehicle.manufacturer = normalize(&self.vehicle.manufacturer);
        self.vehicle.model = self.vehicle.model.as_ref().map(|model| normalize(&model));
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateGatePassRequest {
    pub id: Option<Cow<'static, str>>,
    pub owner: UpdateGatePassOwnerRequest,
    pub updated_by: Option<Cow<'static, str>>,
}

impl UpdateGatePassRequest {
    pub fn normalize(&mut self) {
        self.owner.title = normalize(&self.owner.title);
        self.owner.unit = uppercase(&self.owner.unit);
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GatePassOwner {
    pub last_name: Cow<'static, str>,
    pub first_name: Cow<'static, str>,
    pub middle_name: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub unit: Cow<'static, str>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateGatePassOwnerRequest {
    pub title: Cow<'static, str>,
    pub unit: Cow<'static, str>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GatePassVehicle {
    pub number_plate: Cow<'static, str>,
    pub vin_code: Option<Cow<'static, str>>,
    pub manufacturer: Cow<'static, str>,
    pub model: Option<Cow<'static, str>>,
    pub color: VehicleColor,
    pub body_type: VehicleBodyType,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VehicleBodyType {
    #[default]
    Sedan,
    Hatchback,
    Wagon,
    Pickup,
    SUV,
    Coupe,
    Truck,
    Bus,
    Minibus,
    Motorbike,
    Scooter,
}

impl VehicleBodyType {
    pub fn values() -> Vec<VehicleBodyType> {
        vec![
            VehicleBodyType::Sedan,
            VehicleBodyType::Hatchback,
            VehicleBodyType::Wagon,
            VehicleBodyType::Pickup,
            VehicleBodyType::SUV,
            VehicleBodyType::Coupe,
            VehicleBodyType::Truck,
            VehicleBodyType::Bus,
            VehicleBodyType::Minibus,
            VehicleBodyType::Motorbike,
            VehicleBodyType::Scooter,
        ]
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VehicleColor {
    #[default]
    White,
    Beige,
    Grey,
    DarkGrey,
    Blue,
    DarkBlue,
    Purple,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    Orange,
    Brown,
    Black,
}

impl VehicleColor {
    pub fn values() -> Vec<VehicleColor> {
        vec![
            VehicleColor::White,
            VehicleColor::Beige,
            VehicleColor::Grey,
            VehicleColor::DarkGrey,
            VehicleColor::Blue,
            VehicleColor::DarkBlue,
            VehicleColor::Purple,
            VehicleColor::Red,
            VehicleColor::DarkRed,
            VehicleColor::Green,
            VehicleColor::DarkGreen,
            VehicleColor::Yellow,
            VehicleColor::Orange,
            VehicleColor::Brown,
            VehicleColor::Black,
        ]
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchGatePassRequest {
    pub last_name: Option<Cow<'static, str>>,
    pub number_plate: Option<Cow<'static, str>>,
    pub number_of_results: Option<i32>,
}

impl SearchGatePassRequest {
    pub fn normalize(&mut self) {
        self.last_name = self
            .last_name
            .as_ref()
            .map(|last_name| normalize(&last_name));
        self.number_plate = self
            .number_plate
            .as_ref()
            .map(|number_plate| uppercase(&number_plate));
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SyncGatePassRequest {
    pub last_synced_at: Option<Cow<'static, str>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SyncGatePassResponse {
    pub last_synced_at: Cow<'static, str>,
    pub gate_passes: Vec<SyncGatePass>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SyncGatePass {
    pub id: Cow<'static, str>,
    pub expired_at: Cow<'static, str>,
    pub deleted: bool,
    pub owner: Option<GatePassOwner>,
    pub vehicle: Option<GatePassVehicle>,
    pub allow_any_vehicle: bool,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GatePassValidationResult {
    #[default]
    Valid,
    Deleted,
    Expired,
    NotFound,
}

fn normalize(string: &Cow<'static, str>) -> Cow<'static, str> {
    let string = string.to_lowercase();
    let mut chars = string.chars();
    match chars.next() {
        None => Cow::Borrowed(""),
        Some(first) => Cow::Owned(first.to_uppercase().collect::<String>() + chars.as_str()),
    }
}

fn uppercase(string: &Cow<'static, str>) -> Cow<'static, str> {
    Cow::Owned(string.to_uppercase())
}
