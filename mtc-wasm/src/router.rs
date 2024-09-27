use crate::prelude::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(Layout)]
    #[route("/not-found")]
    #[redirect("/:..segments", | segments: Vec < String > | Route::NotFound {})]
    NotFound {},
    #[route("/")]
    Home {},
    #[route("/sign-in")]
    SignIn {},    
    #[route("/change-password")]
    ChangePassword {},
    #[route("/administrator")]
    Administrator {},
    #[route("/administrator/permissions")]
    Permissions {},
    #[route("/administrator/permissions/create")]
    PermissionCreate {},
    #[route("/administrator/groups")]
    Groups {},
    #[route("/administrator/groups/edit/:id")]
    GroupEdit { id: String },
    #[route("/administrator/roles")]
    Roles {},
    #[route("/administrator/roles/edit/:id")]
    RoleEdit { id: String },
    #[route("/administrator/users")]
    Users {},
    #[route("/administrator/users/edit/:id")]
    UserEdit { id: String },
}