use validator::ValidationError;

pub fn not_blank(string: &str) -> Result<(), ValidationError> {
    if string.trim().is_empty() {
        return Err(ValidationError::new("cannot be blank"));
    }
    Ok(())
}