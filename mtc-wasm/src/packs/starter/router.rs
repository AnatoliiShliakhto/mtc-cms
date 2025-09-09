use super::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(Layout)]
    #[route("/:..route")]
    CustomRouter { route: Vec<String> }
}

/// This is a catch-all route for all routes that are not explicitly defined in other routes.
///
/// It uses the route path as a string to determine which component to render.
#[component]
fn CustomRouter(#[props(into)] route: Vec<String>) -> Element {
    jsFfiDestroyCkEditor();
    jsFfiDestroyHtml5QrcodeScanner();

    if route.is_empty() {
        return rsx! { Home {} };
    }

    let static_route = route.join("/");

    match static_route.as_str() {
        "home" => return rsx! { Home {} },
        "administrator" => return rsx! { Administrator {} },
        "auth/sign-in" => return rsx! { SignIn {} },
        "auth/qr-sign-in" => return rsx! { QrSignIn {} },
        "auth/change-password" => return rsx! { ChangePassword {} },
        "auth/linking-qr-code" => return rsx! { LinkingQrCode {} },
        "personnel" => return rsx! { Personnel {} },
        "personnel/add" => return rsx! { PersonnelAdd {} },
        "administrator/permissions" => return rsx! { Permissions {} },
        "administrator/groups" => return rsx! { Groups {} },
        "administrator/roles" => return rsx! { Roles {} },
        "administrator/users" => return rsx! { Users {} },
        "administrator/schemas" => return rsx! { Schemas {} },
        "administrator/permission/create" => return rsx! { PermissionCreate {} },
        "administrator/js" => return rsx! { JsExec {} },
        "application/data" => return rsx! { AppData {} },
        "administrator/gate-passes" => return rsx! { GatePasses {} },
        "gate-pass-validation-scans" => return rsx! { GatePassScan {} },
        _ => {}
    }

    match route.len() {
        2 => match route[0].as_str() {
            "content" => rsx! { ContentList { schema: route[1].clone() } },
            "search" => rsx! { Search { pattern: route[1].clone() } },
            _ => rsx! { NotFound {} },
        },
        3 => match route[0].as_str() {
            "content" => rsx! { ContentView { schema: route[1].clone(), slug: route[2].clone() } },
            "search" => rsx! { Search { pattern: route[1].clone() } },
            "editor" => rsx! { ContentEdit { schema: route[1].clone(), slug: route[2].clone() } },
            "administrator" => match route[1].as_str() {
                "group" => rsx! { GroupEdit { id: route[2].clone() } },
                "role" => rsx! { RoleEdit { id: route[2].clone() } },
                "user" => rsx! { UserEdit { id: route[2].clone() } },
                "schema" => rsx! { SchemaEdit { id: route[2].clone() } },
                "gate-passes" => rsx! { GatePassEdit { id: route[2].clone() } },
                _ => rsx! { NotFound {} },
            },
            "gate-pass-validation-scans" => match route[1].as_str() {
                "errors" => rsx! { GatePassScanError { error: route[2].clone() } },
                "results" => rsx! { GatePassScanResult { id: route[2].clone() } },
                _ => rsx! { NotFound {} },
            },

            _ => rsx! { NotFound {} },
        },
        4 => match route[0].as_str() {
            "content" => {
                rsx! { ContentView { schema: route[1].clone(), slug: route[2].clone(), arg: route[3].clone() } }
            }
            _ => rsx! { NotFound {} },
        },
        5 => {
            if route[0].eq("course") && route[1].eq("editor") {
                rsx! {
                    CourseEdit {
                        slug: route[2].clone(),
                        id: route[3].clone().parse::<usize>().unwrap_or_default(),
                        is_new: route[4].eq("true"),
                    }
                }
            } else {
                rsx! { NotFound {} }
            }
        }
        _ => rsx! { NotFound {} },
    }
}
