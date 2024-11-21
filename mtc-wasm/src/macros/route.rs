#[macro_export]
macro_rules! route {
    () => {
        {
            Route::CustomRouter { route: vec!["".to_string()] }
        }
    };

    ($first:expr) => {
        {
            Route::CustomRouter { route: vec![$first.to_string()] }
        }
    };

    ($first:expr, $second:expr) => {
        {
            Route::CustomRouter { route: vec![$first.to_string(), $second.to_string()] }
        }
    };

    ($first:expr, $second:expr, $third:expr) => {
        {
            Route::CustomRouter { route: vec![$first.to_string(), $second.to_string(), $third.to_string()] }
        }
    };

    ($first:expr, $second:expr, $third:expr, $fourth:expr) => {
        {
            Route::CustomRouter { route: vec![$first.to_string(), $second.to_string(), $third.to_string(), $fourth.to_string()] }
        }
    };

    ($first:expr, $second:expr, $third:expr, $fourth:expr, $fifth:expr) => {
        {
            Route::CustomRouter { route: vec![$first.to_string(), $second.to_string(), $third.to_string(), $fourth.to_string(), $fifth.to_string()] }
        }
    };
}