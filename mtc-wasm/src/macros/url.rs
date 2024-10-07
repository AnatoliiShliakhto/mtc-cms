#[macro_export]
macro_rules! url {
    ($first:expr) => {
        {
            [API_ENDPOINT, $first].join("/").into()
        }
    };

    ($first:expr, $second:expr) => {
        {
            [API_ENDPOINT, $first, $second].join("/").into()
        }
    };

    ($first:expr, $second:expr, $third:expr) => {
        {
            [API_ENDPOINT, $first, $second, $third].join("/").into()
        }
    };
}