use crate::prelude::not_blank;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use validator::Validate;

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GatePass {
    pub id: Cow<'static, str>,
    pub number: Cow<'static, str>,
    pub expired_at: Cow<'static, str>,
    pub deleted: bool,
    pub owner: GatePassOwner,
    pub allow_any_vehicle: bool,
    pub vehicles: Vec<GatePassVehicle>,
    pub block: Option<GatePassBlock>,
    pub created_at: Cow<'static, str>,
    pub updated_at: Cow<'static, str>,
    pub created_by: Cow<'static, str>,
    pub updated_by: Cow<'static, str>,
}

impl GatePass {
    pub fn expired(&self) -> bool {
        expired(&self.expired_at)
    }

    pub fn blocked(&self) -> bool {
        self.block
            .as_ref()
            .filter(|block| !block.expired())
            .is_some()
    }

    pub fn require_first_number_plate_type(&self) -> GatePassVehicleNumberPlateType {
        self.require_first_vehicle().number_plate_type()
    }

    pub fn require_first_vehicle(&self) -> &GatePassVehicle {
        self.vehicles
            .first()
            .expect("active gate pass should have at least one vehicle")
    }

    pub fn first_vehicle(&self) -> Option<&GatePassVehicle> {
        self.vehicles.first()
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct CreateGatePassRequest {
    #[validate(custom(function = "not_blank"))]
    pub expired_at: Cow<'static, str>,
    #[validate(nested)]
    pub owner: GatePassOwner,
    pub allow_any_vehicle: bool,
    #[validate(nested)]
    #[validate(length(min = 1))]
    pub vehicles: Vec<GatePassVehicle>,
    pub created_by: Option<Cow<'static, str>>,
    pub updated_by: Option<Cow<'static, str>>,
}

impl CreateGatePassRequest {
    pub fn normalize(&mut self) {
        self.owner.last_name = normalize(&self.owner.last_name);
        self.owner.first_name = normalize(&self.owner.first_name);
        self.owner.middle_name = normalize(&self.owner.middle_name);
        self.owner.unit = uppercase(&self.owner.unit);
        self.vehicles.iter_mut().for_each(|vehicle| {
            vehicle.number_plate = Cow::Owned(uppercase(&vehicle.number_plate).replace(" ", ""));
            vehicle.vin_code = vehicle
                .vin_code
                .as_ref()
                .map(|vin_code| uppercase(&vin_code));
            vehicle.manufacturer = normalize(&vehicle.manufacturer);
            vehicle.model = vehicle.model.as_ref().map(|model| normalize(&model));
        });
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct CreateGatePassBatchRequest {
    #[validate(nested)]
    pub requests: Vec<CreateGatePassRequest>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateGatePassBatchResponse {
    pub created_gate_passes: Vec<GatePass>,
    pub failed_requests: Vec<CreateGatePassRequest>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct UpdateGatePassRequest {
    pub id: Option<Cow<'static, str>>,
    #[validate(nested)]
    pub owner: UpdateGatePassOwnerRequest,
    pub updated_by: Option<Cow<'static, str>>,
}

impl UpdateGatePassRequest {
    pub fn normalize(&mut self) {
        self.owner.unit = uppercase(&self.owner.unit);
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct GatePassOwner {
    #[validate(custom(function = "not_blank"))]
    pub last_name: Cow<'static, str>,
    #[validate(custom(function = "not_blank"))]
    pub first_name: Cow<'static, str>,
    #[validate(custom(function = "not_blank"))]
    pub middle_name: Cow<'static, str>,
    pub title: GatePassOwnerTitle,
    #[validate(custom(function = "not_blank"))]
    pub unit: Cow<'static, str>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GatePassOwnerTitle {
    #[default]
    Soldier,
    SeniorSoldier,
    JuniorSergeant,
    Sergeant,
    SeniorSergeant,
    ChiefSergeant,
    StaffSergeant,
    MasterSergeant,
    SeniorMasterSergeant,
    ChiefMasterSergeant,
    JuniorLieutenant,
    Lieutenant,
    SeniorLieutenant,
    Captain,
    Major,
    LieutenantColonel,
    Colonel,
    BrigadierGeneral,
    MajorGeneral,
    LieutenantGeneral,
    General,
    GeneralOfTheArmyOfUkraine,
}

impl GatePassOwnerTitle {
    pub fn values() -> Vec<GatePassOwnerTitle> {
        vec![
            GatePassOwnerTitle::Soldier,
            GatePassOwnerTitle::SeniorSoldier,
            GatePassOwnerTitle::JuniorSergeant,
            GatePassOwnerTitle::Sergeant,
            GatePassOwnerTitle::SeniorSergeant,
            GatePassOwnerTitle::ChiefSergeant,
            GatePassOwnerTitle::StaffSergeant,
            GatePassOwnerTitle::MasterSergeant,
            GatePassOwnerTitle::SeniorMasterSergeant,
            GatePassOwnerTitle::ChiefMasterSergeant,
            GatePassOwnerTitle::JuniorLieutenant,
            GatePassOwnerTitle::Lieutenant,
            GatePassOwnerTitle::SeniorLieutenant,
            GatePassOwnerTitle::Captain,
            GatePassOwnerTitle::Major,
            GatePassOwnerTitle::LieutenantColonel,
            GatePassOwnerTitle::Colonel,
            GatePassOwnerTitle::BrigadierGeneral,
            GatePassOwnerTitle::MajorGeneral,
            GatePassOwnerTitle::LieutenantGeneral,
            GatePassOwnerTitle::General,
            GatePassOwnerTitle::GeneralOfTheArmyOfUkraine,
        ]
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct UpdateGatePassOwnerRequest {
    pub title: GatePassOwnerTitle,
    #[validate(custom(function = "not_blank"))]
    pub unit: Cow<'static, str>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct GatePassVehicle {
    #[validate(custom(function = "not_blank"))]
    pub number_plate: Cow<'static, str>,
    pub vin_code: Option<Cow<'static, str>>,
    #[validate(custom(function = "not_blank"))]
    pub manufacturer: Cow<'static, str>,
    pub model: Option<Cow<'static, str>>,
    pub color: VehicleColor,
    pub body_type: VehicleBodyType,
}

impl GatePassVehicle {
    pub fn number_plate_type(&self) -> GatePassVehicleNumberPlateType {
        let number_plate = self.number_plate.replace(" ", "");
        // 1234 А5
        let military_format = Regex::new(r"^\d{4}\D\d$").unwrap();
        // АН 1234 АС
        let civil_format = Regex::new(r"^\D{2}\d{4}\D{2}$").unwrap();
        if military_format.is_match(&number_plate) {
            let numbers = number_plate.chars().take(4).collect();
            let letters = number_plate.chars().skip(4).take(2).collect();
            GatePassVehicleNumberPlateType::MILITARY(Cow::Owned(numbers), Cow::Owned(letters))
        } else if civil_format.is_match(&number_plate) {
            let start_letters = number_plate.chars().take(2).collect();
            let numbers = number_plate.chars().skip(2).take(4).collect();
            let end_letters = number_plate.chars().skip(6).take(2).collect();
            GatePassVehicleNumberPlateType::CIVIL(
                Cow::Owned(start_letters),
                Cow::Owned(numbers),
                Cow::Owned(end_letters),
            )
        } else {
            GatePassVehicleNumberPlateType::UNKNOWN(Cow::Owned(number_plate))
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GatePassVehicleNumberPlateType {
    UNKNOWN(Cow<'static, str>),
    MILITARY(Cow<'static, str>, Cow<'static, str>),
    CIVIL(Cow<'static, str>, Cow<'static, str>, Cow<'static, str>),
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateGatePassBlockRequest {
    pub id: Option<Cow<'static, str>>,
    pub block: Option<GatePassBlock>,
    pub updated_by: Option<Cow<'static, str>>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct GatePassBlock {
    #[validate(custom(function = "not_blank"))]
    pub expired_at: Cow<'static, str>,
    #[validate(custom(function = "not_blank"))]
    pub reason: Cow<'static, str>,
}

impl GatePassBlock {
    pub fn expired(&self) -> bool {
        expired(&self.expired_at)
    }
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
    pub number: Cow<'static, str>,
    pub expired_at: Cow<'static, str>,
    pub deleted: bool,
    pub owner: Option<GatePassOwner>,
    pub allow_any_vehicle: bool,
    pub vehicles: Vec<GatePassVehicle>,
    pub block: Option<GatePassBlock>,
}

impl SyncGatePass {
    pub fn blocked(&self) -> bool {
        self.block
            .as_ref()
            .filter(|block| !block.expired())
            .is_some()
    }

    pub fn expired(&self) -> bool {
        expired(&self.expired_at)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GatePassValidationResult {
    #[default]
    Valid,
    Blocked(Cow<'static, str>),
    Expired,
    Deleted,
    NotFound,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq, Validate)]
pub struct SendGatePassEmailRequest {
    #[validate(email)]
    pub recipient_email: Cow<'static, str>,
}

fn normalize(string: &Cow<'static, str>) -> Cow<'static, str> {
    let string = string.trim().to_lowercase();
    let mut chars = string.chars();
    match chars.next() {
        None => Cow::Borrowed(""),
        Some(first) => Cow::Owned(first.to_uppercase().collect::<String>() + chars.as_str()),
    }
}

fn uppercase(string: &Cow<'static, str>) -> Cow<'static, str> {
    Cow::Owned(string.trim().to_uppercase())
}

fn expired(expired_at: &Cow<'static, str>) -> bool {
    expired_at
        .parse::<DateTime<Utc>>()
        .ok()
        .map(|expired_at| expired_at <= Utc::now())
        .unwrap_or(true)
}
