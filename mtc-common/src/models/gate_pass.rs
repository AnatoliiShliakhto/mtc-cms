use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GatePass {
    pub id: Cow<'static, str>,
    pub expired_at: Cow<'static, str>,
    pub deleted: bool,
    pub owner: GatePassOwner,
    pub vehicle: GatePassVehicle,
    pub created_at: Cow<'static, str>,
    pub updated_at: Cow<'static, str>,
    pub created_by: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateGatePassRequest {
    pub id: Option<Cow<'static, str>>,
    pub expired_at: Cow<'static, str>,
    pub owner: GatePassOwner,
    pub vehicle: GatePassVehicle,
    pub created_by: Option<Cow<'static, str>>,
    pub updated_by: Option<Cow<'static, str>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateGatePassRequest {
    pub id: Option<Cow<'static, str>>,
    pub owner: UpdateGatePassOwnerRequest,
    pub updated_by: Option<Cow<'static, str>>,
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
    pub body_type: Option<VehicleBodyType>,
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

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchGatePassRequest {
    pub last_name: Option<Cow<'static, str>>,
    pub first_name: Option<Cow<'static, str>>,
    pub middle_name: Option<Cow<'static, str>>,
    pub number_plate: Option<Cow<'static, str>>,
    pub manufacturer: Option<Cow<'static, str>>,
    pub color: Option<Cow<'static, str>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SyncGatePassRequest {
    pub updated_after: Option<Cow<'static, str>>,
    pub updated_before: Option<Cow<'static, str>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SyncGatePassResponse {
    pub updated_after: Option<Cow<'static, str>>,
    pub updated_before: Option<Cow<'static, str>>,
    pub gate_passes: Vec<GatePass>,
}
