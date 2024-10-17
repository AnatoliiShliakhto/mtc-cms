#[macro_export]
macro_rules! check_permission {
    ($permission:ident) => {
        if !use_auth_state()().has_permission($permission) {
            return rsx! { AccessForbidden {} }
        }
    };
}

#[macro_export]
macro_rules! check_role {
    ($role:ident) => {
        if !use_auth_state()().has_role($role) {
            return rsx! { AccessForbidden {} }
        }
    };
}

#[macro_export]
macro_rules! check_response {
    ($response:ident) => {
        if $response().is_null() {
            return rsx! { SomethingWrong {} }
        }
    };

    ($response:ident, $future:ident) => {
        if $response().is_null() {
            return rsx! { SomethingWrong { future: Some($future) } }
        }
    };
}