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

#[macro_export]
macro_rules! state_fn {
    ($signal:tt) => {
        {
            (use_state().$signal())
        }
    };
}