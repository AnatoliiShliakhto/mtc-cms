/// Translates the given string using the current language.
#[macro_export]
macro_rules! t {
    ($id:expr) => {{
        let translation_id = $id;
        state!(i18n)
            .get(translation_id)
            .map(|value| value.to_string())
            .unwrap_or_else(|| translation_id.to_string())
    }};
}