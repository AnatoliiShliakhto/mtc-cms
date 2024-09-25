#[macro_export]
macro_rules! fail {
    ($future:ident) => {
        return rsx! { SomethingWrong { future: Some($future) } }
    };
}