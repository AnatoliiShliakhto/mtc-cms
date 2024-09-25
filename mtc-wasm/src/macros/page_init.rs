#[macro_export]
macro_rules! page_init {
    ($menu:expr) => {
        build_breadcrumbs($menu);
    };

    ($menu:expr, $permission:expr, $auth_state:ident) => {
        build_breadcrumbs($menu);

        if !$auth_state().has_permission($permission) {
            return rsx! { AccessForbidden {} }
        }
    };
}