#[macro_export]
macro_rules! t {
    ($id:expr) => {
        {
            state!(i18n).get($id).unwrap_or(&Cow::Owned($id.to_string())).to_owned()
        }
    };
}