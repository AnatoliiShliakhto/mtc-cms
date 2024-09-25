#[macro_export]
macro_rules! url {
    ($first:expr) => {
        {
            [API_URL, $first].join("/").into()
        }
    };

    ($first:expr, $second:expr) => {
        {
            [API_URL, $first, $second].join("/").into()
        }
    };

    ($first:expr, $second:expr, $third:expr) => {
        {
            [API_URL, $first, $second, $third].join("/").into()
        }
    };
}