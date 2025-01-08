/// Checks if the user has the given permission.
#[macro_export]
macro_rules! check_permission {
    ($permission:ident) => {
        if !state!(auth).has_permission($permission) {
            return rsx! { AccessForbidden {} }
        }
    };
}

/// Checks if the user has the given role.
#[macro_export]
macro_rules! check_role {
    ($role:ident) => {
        if !state!(auth).has_role($role) {
            return rsx! { AccessForbidden {} }
        }
    };
}

/// Checks if the response is null.
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