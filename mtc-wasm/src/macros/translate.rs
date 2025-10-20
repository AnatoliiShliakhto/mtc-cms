/// Translates the given string using the current language.
#[macro_export]
macro_rules! t {
    ($id:expr) => {
        state!(i18n).get($id).map(|value| value.to_string()).unwrap_or_else(|| $id.to_string())
    };
}