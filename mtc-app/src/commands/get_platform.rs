use super::*;

#[command(async)]
pub fn get_platform() -> Result<String, Error> {
    let platform = if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "android") {
        "android".to_string()
    } else {
        "linux".to_string()
    };

    Ok(platform)
}
