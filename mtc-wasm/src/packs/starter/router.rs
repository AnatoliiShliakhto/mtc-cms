use super::*;

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(Layout)]
    #[route("/:..route")]
    CustomRouter { route: Vec<String> }
}

#[component]
fn CustomRouter(
    #[props(into)]
    route: Vec<String>,
) -> Element {
    if route.is_empty() { return rsx! { Home {} } }

    let static_route = route.join("/");

    match static_route.as_str() {
        "home" => return rsx! { Home {} },
        "administrator" => return rsx! { Administrator {} },
        "sign-in" => return rsx! { SignIn {} },
        "change-password" => return rsx! { ChangePassword {} },
        "administrator/permissions" => return rsx! { Permissions {} },
        "administrator/groups" => return rsx! { Groups {} },
        "administrator/roles" => return rsx! { Roles {} },
        "administrator/users" => return rsx! { Users {} },
        "administrator/schemas" => return rsx! { Schemas {} },
        "administrator/permission/create"=> return rsx! { PermissionCreate {} },
        _ => {}
    }

    match route.len() {
        2 => {
            match route[0].as_str() {
                "content" => rsx! { ContentList { schema: route[1].clone() } },
                "search" => rsx! { Search { pattern: route[1].clone() } },
                _ => rsx! { NotFound {} },
            }
        }
        3 => {
            match route[0].as_str() {
                "content" => rsx! { ContentView { schema: route[1].clone(), slug: route[2].clone() } },
                "search" => rsx! { Search { pattern: route[1].clone() } },
                "editor" => rsx! { ContentEdit { schema: route[1].clone(), slug: route[2].clone() } },
                "administrator" => {
                    match route[1].as_str() {
                        "group" => rsx! { GroupEdit { id: route[2].clone() } },
                        "role" => rsx! { RoleEdit { id: route[2].clone() } },
                        "user" => rsx! { UserEdit { id: route[2].clone() } },
                        "schema" => rsx! { SchemaEdit { id: route[2].clone() } },
                        _ => rsx! { NotFound {} },
                    }
                }
                _ => rsx! { NotFound {} },
            }
        }
        4 => {
            match route[0].as_str() {
                "content" => rsx! { ContentView { schema: route[1].clone(), slug: route[2].clone(), arg: route[3].clone() } },
                _ => rsx! { NotFound {} },
            }
        }
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