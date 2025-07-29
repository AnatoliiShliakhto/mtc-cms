/// Build API URL
#[macro_export]
macro_rules! url {
    ($first:expr) => {
        {
            [&API_ENDPOINT, $first].join("/")
        }
    };

    ($first:expr, $second:expr) => {
        {
            [&API_ENDPOINT, $first, $second].join("/")
        }
    };

    ($first:expr, $second:expr, $third:expr) => {
        {
            [&API_ENDPOINT, $first, $second, $third].join("/")
        }
    };
}