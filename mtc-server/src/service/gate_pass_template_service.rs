use crate::error::Error;
use crate::prelude::GenericError::InternalError;
use crate::prelude::{error};
use askama::Template;
use chrono::{DateTime, Datelike, Utc};
use itertools::Itertools;
use mtc_common::prelude::{
    GatePass, GatePassOwnerTitle, GatePassVehicleNumberPlateType, VehicleColor,
};

#[derive(Template)]
#[template(path = "gate_pass_print.html")]
pub struct GatePassPrintHtmlTemplate {
    pub manual_two_side_printing: bool,
    pub front_chunks: Vec<Vec<String>>,
    pub back: String,
}

pub fn gate_pass_print_html(
    manual_two_side_printing: bool,
    fronts: Vec<String>,
    back: String,
) -> crate::prelude::Result<String> {
    let front_chunks = fronts
        .into_iter()
        .chunks(4)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<_>>())
        .collect();
    let gate_pass_print_html_template = GatePassPrintHtmlTemplate {
        manual_two_side_printing,
        front_chunks,
        back,
    };
    gate_pass_print_html_template.render().map_err(|error| {
        error!("failed to render gate pass print html template: {error:?}",);
        Error::GenericError(InternalError)
    })
}

#[derive(Template)]
#[template(path = "gate_pass_front.html")]
pub struct GatePassFrontHtmlTemplate<'a> {
    pub number: &'a str,
    pub vehicle_number_plate_zsu: bool,
    pub vehicle_number_plate_civil: &'a str,
    pub vehicle_number_plate_numbers_zsu: &'a str,
    pub vehicle_number_plate_letters_zsu: &'a str,
    pub qr_code: &'a str,
    pub owner_details: &'a str,
    pub vehicle_details: &'a str,
    pub expired_at_details: &'a str,
}

pub fn gate_pass_front_html(
    gate_pass: &GatePass,
    qr_code_png_base64: &String,
) -> crate::prelude::Result<String> {
    let mut vehicle_number_plate_zsu = false;
    let mut vehicle_number_plate_civil = "".to_string();
    let mut vehicle_number_plate_numbers_zsu = "".to_string();
    let mut vehicle_number_plate_letters_zsu = "".to_string();
    match gate_pass.require_first_number_plate_type() {
        GatePassVehicleNumberPlateType::UNKNOWN(number) => {
            vehicle_number_plate_civil = number.to_string()
        }
        GatePassVehicleNumberPlateType::CIVIL(start_letters, numbers, end_letters) => {
            vehicle_number_plate_civil = format!("{start_letters} {numbers} {end_letters}")
        }
        GatePassVehicleNumberPlateType::MILITARY(numbers, letters) => {
            vehicle_number_plate_zsu = true;
            vehicle_number_plate_numbers_zsu = numbers.to_string();
            vehicle_number_plate_letters_zsu = letters.to_string();
        }
    }
    let gate_pass_front_html_template = GatePassFrontHtmlTemplate {
        number: &gate_pass.number,
        vehicle_number_plate_zsu,
        vehicle_number_plate_civil: vehicle_number_plate_civil.as_str(),
        vehicle_number_plate_numbers_zsu: vehicle_number_plate_numbers_zsu.as_ref(),
        vehicle_number_plate_letters_zsu: vehicle_number_plate_letters_zsu.as_ref(),
        qr_code: &qr_code_png_base64,
        owner_details: &owner_details(&gate_pass),
        vehicle_details: &vehicle_details(&gate_pass),
        expired_at_details: &expired_at_details(&gate_pass)?,
    };

    gate_pass_front_html_template.render().map_err(|error| {
        error!("failed to render gate pass front html template: {error:?}",);
        Error::GenericError(InternalError)
    })
}

fn owner_details(gate_pass: &GatePass) -> String {
    let title = match gate_pass.owner.title {
        GatePassOwnerTitle::Soldier => "солдат",
        GatePassOwnerTitle::SeniorSoldier => "ст. солдат",
        GatePassOwnerTitle::JuniorSergeant => "мол. сержант",
        GatePassOwnerTitle::Sergeant => "сержант",
        GatePassOwnerTitle::SeniorSergeant => "ст. сержант",
        GatePassOwnerTitle::ChiefSergeant => "гол. сержант",
        GatePassOwnerTitle::StaffSergeant => "штаб-сержант",
        GatePassOwnerTitle::MasterSergeant => "майстер-сержант",
        GatePassOwnerTitle::SeniorMasterSergeant => "ст. майстер-сержант",
        GatePassOwnerTitle::ChiefMasterSergeant => "гол. майстер-сержант",
        GatePassOwnerTitle::JuniorLieutenant => "мол. лейтенант",
        GatePassOwnerTitle::Lieutenant => "лейтенант",
        GatePassOwnerTitle::SeniorLieutenant => "ст. лейтенант",
        GatePassOwnerTitle::Captain => "капітан",
        GatePassOwnerTitle::Major => "майор",
        GatePassOwnerTitle::LieutenantColonel => "підполковник",
        GatePassOwnerTitle::Colonel => "полковник",
        GatePassOwnerTitle::BrigadierGeneral => "бригадний генерал",
        GatePassOwnerTitle::MajorGeneral => "генерал-майор",
        GatePassOwnerTitle::LieutenantGeneral => "генерал-лейтенант",
        GatePassOwnerTitle::General => "генерал",
        GatePassOwnerTitle::GeneralOfTheArmyOfUkraine => "генерал армії",
    };
    format!(
        "{} {} {} {}",
        title,
        gate_pass.owner.last_name.to_uppercase(),
        gate_pass.owner.first_name,
        gate_pass.owner.middle_name,
    )
}

fn vehicle_details(gate_pass: &GatePass) -> String {
    let color = match gate_pass.vehicles.first().unwrap().color {
        VehicleColor::White => "білий",
        VehicleColor::Beige => "бежевий",
        VehicleColor::Grey => "сірий",
        VehicleColor::DarkGrey => "темно-сірий",
        VehicleColor::Blue => "синій",
        VehicleColor::DarkBlue => "темно-синій",
        VehicleColor::Purple => "фіолетовий",
        VehicleColor::Red => "красний",
        VehicleColor::DarkRed => "темно-красний",
        VehicleColor::Green => "зелений",
        VehicleColor::DarkGreen => "темно-зелений",
        VehicleColor::Yellow => "жовтий",
        VehicleColor::Orange => "помаранчевий",
        VehicleColor::Brown => "коричневий",
        VehicleColor::Black => "чорний",
    };
    format!(
        "{} {}, {}, {}",
        gate_pass.vehicles.first().unwrap().manufacturer,
        gate_pass
            .vehicles
            .first()
            .unwrap()
            .model
            .clone()
            .unwrap_or_default(),
        gate_pass.vehicles.first().unwrap().number_plate,
        color
    )
}

fn expired_at_details(gate_pass: &GatePass) -> crate::prelude::Result<String> {
    let expired_at = gate_pass
        .expired_at
        .parse::<DateTime<Utc>>()
        .map_err(|error| {
            error!("{}", error);
            InternalError
        })?;
    let month = match expired_at.month() {
        1 => "січня",
        2 => "лютого",
        3 => "березня",
        4 => "квітня",
        5 => "травня",
        6 => "червня",
        7 => "липня",
        8 => "серпня",
        9 => "вересня",
        10 => "жовтня",
        11 => "листопада",
        12 => "грудня",
        _ => "невідомо",
    };
    Ok(format!(
        "{} {} {} р.",
        expired_at.day(),
        month,
        expired_at.year()
    ))
}

#[derive(Template)]
#[template(path = "gate_pass_back.html")]
pub struct GatePassBackHtmlTemplate {}

pub fn gate_pass_back_html() -> crate::prelude::Result<String> {
    GatePassBackHtmlTemplate {}.render().map_err(|error| {
        error!("failed to render gate pass back html template: {error:?}",);
        Error::GenericError(InternalError)
    })
}

#[derive(Template)]
#[template(path = "gate_pass_email.html")]
pub struct GatePassEmailHtmlTemplate<'a> {
    pub number: &'a str,
}

pub fn gate_pass_email_html(gate_pass: &GatePass) -> crate::prelude::Result<String> {
    GatePassEmailHtmlTemplate {
        number: gate_pass.number.as_ref(),
    }
    .render()
    .map_err(|error| {
        error!("failed to render gate pass email html template: {error:?}",);
        Error::GenericError(InternalError)
    })
}
