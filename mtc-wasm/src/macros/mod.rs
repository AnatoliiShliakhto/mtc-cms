mod translate;
mod url;
mod breadcrumbs;
mod checks;
mod requests;
mod dialogs;
mod route;

pub mod prelude {
    pub use crate::{
        url,
        route,
        t,
        breadcrumbs,
        value_future,
        check_response,
        check_permission,
        check_role,
        get_request,
        delete_request,
        post_request,
        patch_request,
        value_request,
        close_dialog,
        alert_dialog,
        success_dialog,
        error_dialog,
        info_dialog,
        warning_dialog,
    };
}