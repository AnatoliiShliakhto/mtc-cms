/// Build breadcrumbs
#[macro_export]
macro_rules! breadcrumbs {
    () => {
        use_effect(|| { state!(set_breadcrumbs, vec![]) });
    };

    ($slug:expr) => {
        build_breadcrumbs($slug)
    };
}