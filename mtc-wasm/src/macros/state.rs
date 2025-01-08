/// Get the value of an application state signal
#[macro_export]
macro_rules! state {
    ($signal:tt) => {
        {
            (use_state().$signal())()
        }
    };

    ($signal:tt, $value:expr) => {
        {
            use_state().$signal($value)
        }
    };
}

/// Run a function on an application state signal
#[macro_export]
macro_rules! state_fn {
    ($signal:tt) => {
        {
            (use_state().$signal())
        }
    };
}