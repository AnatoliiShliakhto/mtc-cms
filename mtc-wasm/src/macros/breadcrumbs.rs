#[macro_export]
macro_rules! breadcrumbs {
    () => {
        use_effect(|| { use_breadcrumbs().set(vec![]) });
    };

    ($slug:expr) => {
        build_breadcrumbs($slug)
    };
}