use super::*;

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
    #[route("/administrator/permission/create")]
    PermissionCreate {},
    #[route("/administrator/groups")]
    Groups {},
    #[route("/administrator/group/edit/:id")]
    GroupEdit { id: String },
    #[route("/administrator/roles")]
    Roles {},
    #[route("/administrator/role/edit/:id")]
    RoleEdit { id: String },
    #[route("/administrator/users")]
    Users {},
    #[route("/administrator/user/edit/:id")]
    UserEdit { id: String },
    #[route("/administrator/schemas")]
    Schemas {},
    #[route("/administrator/schema/edit/:id")]
    SchemaEdit { id: String },
    #[route("/list/:schema")]
    ContentList { schema: String },
    #[route("/view/:schema/:slug")]
    ContentView { schema: String, slug: String },
    #[route("/view/:schema/:slug/:arg")]
    ContentViewWithArg { schema: String, slug: String, arg: String },
    #[route("/edit/:schema/:slug")]
    ContentEdit { schema: String, slug: String },
}