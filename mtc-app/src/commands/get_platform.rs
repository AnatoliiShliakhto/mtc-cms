use super::*;

/// Returns the current platform as a string.
///
/// This function detects the operating system on which it is running
/// and returns a string representation of the platform. The possible
/// return values are "windows", "android", "ios", or "linux".
///
/// # Errors
///
/// Returns an [`Error`] if there is an issue determining the platform.
#[command(rename_all = "snake_case")]
pub fn get_platform() -> Result<String, Error> {
    let platform = if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "android") {
        "android".to_string()
    } else if cfg!(target_os = "ios") {
        "ios".to_string()
    } else {
        "linux".to_string()
    };

    Ok(platform)
}
