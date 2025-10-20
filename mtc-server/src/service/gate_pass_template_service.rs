use crate::prelude::error;
use crate::prelude::GenericError::InternalError;
use chrono::{DateTime, Datelike, Utc};
use mtc_common::prelude::{
    GatePass, GatePassOwnerTitle, GatePassVehicleNumberPlateType, VehicleColor,
};
use std::borrow::Cow;

pub fn gate_pass_front_html(
    gate_pass: &GatePass,
    qr_code_png_base64: &String,
    gate_pass_front_html_template: &Cow<'static, str>,
) -> crate::prelude::Result<String> {
    let mut gate_pass_front_html = gate_pass_front_html_template
        .replace("{{qr_code}}", &qr_code_png_base64)
        .replace("{{number}}", &gate_pass.number)
        .replace("{{owner_details}}", &owner_details(&gate_pass))
        .replace("{{vehicle_details}}", &vehicle_details(&gate_pass))
        .replace("{{expired_at_details}}", &expired_at_details(&gate_pass)?);
    match gate_pass.require_first_number_plate_type() {
        GatePassVehicleNumberPlateType::UNKNOWN(number) => {
            gate_pass_front_html = gate_pass_front_html
                .replace("{{number-plate-class}}", "number-plate")
                .replace("{{number-plate-zsu-class}}", "hidden")
                .replace("{{number_plate}}", &number);
        }
        GatePassVehicleNumberPlateType::CIVIL(start_letters, numbers, end_letters) => {
            gate_pass_front_html = gate_pass_front_html
                .replace("{{number-plate-class}}", "number-plate")
                .replace("{{number-plate-zsu-class}}", "hidden")
                .replace(
                    "{{number_plate}}",
                    &format!("{start_letters} {numbers} {end_letters}"),
                );
        }
        GatePassVehicleNumberPlateType::MILITARY(numbers, letters) => {
            gate_pass_front_html = gate_pass_front_html
                .replace("{{number-plate-class}}", "hidden")
                .replace("{{number-plate-zsu-class}}", "number-plate-zsu")
                .replace("{{numbers}}", &numbers)
                .replace("{{letters}}", &letters);
        }
    }
    Ok(gate_pass_front_html)
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
