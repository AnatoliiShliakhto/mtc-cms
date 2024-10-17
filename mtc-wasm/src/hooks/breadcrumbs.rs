use super::*;

pub fn use_init_breadcrumbs() -> Signal<Vec<(Cow<'static, str>, Cow<'static, str>)>> {
    use_context_provider(UseBreadcrumbs::default).inner
}

pub fn use_breadcrumbs() -> Signal<Vec<(Cow<'static, str>, Cow<'static, str>)>> {
    consume_context::<UseBreadcrumbs>().inner
}

#[derive(Default, Clone, Copy)]
pub struct UseBreadcrumbs {
    inner: Signal<Vec<(Cow<'static, str>, Cow<'static, str>)>>,
}