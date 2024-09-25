use super::*;

pub fn use_init_i18n(language: &str) -> Signal<BTreeMap<Cow<'static, str>, Cow<'static, str>>> {
    let mut strings =
        BTreeMap::<Cow<'static, str>, Cow<'static, str>>::new();

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'=')
        .has_headers(false)
        .flexible(true)
        .trim(csv::Trim::All)
        .from_reader(language.as_bytes());

    for record in reader.deserialize::<I18nStrings>().flatten() {
        strings.insert(record.key, record.value);
    }
    
    use_context_provider(|| UseI18n { inner: Signal::new(strings) }).inner
}

pub fn use_i18n() -> Signal<BTreeMap<Cow<'static, str>, Cow<'static, str>>> {
    consume_context::<UseI18n>().inner
}

#[derive(Default, Clone, Copy)]
pub struct UseI18n {
    inner: Signal<BTreeMap<Cow<'static, str>, Cow<'static, str>>>,
}

#[derive(Deserialize, Clone)]
struct I18nStrings {
    key: Cow<'static, str>,
    value: Cow<'static, str>,
}
