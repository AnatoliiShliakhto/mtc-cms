use crate::prelude::*;

/// Builds breadcrumbs from a given slug
///
/// Given a slug it will generate a vector of `(text, link)` pairs to be used as
/// breadcrumbs in the application. The returned vector is set to the `breadcrumbs`
/// state, which can then be used to display the breadcrumbs in the application.
///
/// The breadcrumbs are generated using the [`use_effect`] hook, which is a hook
/// provided by the `mtc-wasm` library. It takes a closure that returns a vector
/// of `(text, link)` pairs, and sets the [`breadcrumbs`] state to the returned
/// vector.
pub fn build_breadcrumbs(slug: &str) {
    let slug: Cow<'static, str> = slug.to_owned().into();

    use_effect(use_reactive!(|(slug,)| {
        let breadcrumbs: Vec<(Cow<str>, &str)> = match &*slug {
            "menu-sign-in" => vec![(t!("menu-sign-in"), "/auth/sign-in")],
            "menu-change-password" => vec![(t!("menu-change-password"), "/auth/change-password")],
            "menu-search" => vec![(t!("menu-search"), "")],
            "menu-personnel" => vec![(t!("menu-personnel"), "")],
            "menu-linking-qr-code" => vec![(t!("menu-linking-qr-code"), "/auth/linking-qr-code")],

            "menu-administrator" => vec![(t!("menu-administrator"), "/administrator")],
            "menu-permissions" => vec![
                (t!("menu-administrator"), "/administrator"),
                (t!("menu-permissions"), "/administrator/permissions")
            ],
            "menu-groups" => vec![
                (t!("menu-administrator"), "/administrator"),
                (t!("menu-groups"), "/administrator/groups")
            ],

            "menu-roles" => vec![
                (t!("menu-administrator"), "/administrator"),
                (t!("menu-roles"), "/administrator/roles")
            ],

            "menu-users" => vec![
                (t!("menu-administrator"), "/administrator"),
                (t!("menu-users"), "/administrator/users")
            ],

            "menu-schemas" => vec![
                (t!("menu-administrator"), "/administrator"),
                (t!("menu-schemas"), "/administrator/schemas")
            ],

            "menu-page" => vec![
                (t!("menu-content"), ""),
                (t!("menu-page"), "/content/page")
            ],

            "menu-course" => vec![
                (t!("menu-content"), ""),
                (t!("menu-course"), "/content/course")
            ],

            "menu-content-edit" => vec![
                (t!("menu-content"), ""),
                (t!("menu-content-edit"), "")
            ],

            "menu-course-edit" => vec![
                (t!("menu-course"), ""),
                (t!("menu-course-edit"), "")
            ],

            "menu-app-download"
            | "menu-app-data" => vec![
                (t!("menu-application"), ""),
            ],

            _ => vec![],
        };

        state!(set_breadcrumbs, breadcrumbs.into_iter()
            .map(|(key, value)| (key.into(), Cow::Borrowed(value.into())))
            .collect::<Vec<(Cow<'static, str>, Cow<'static, str>)>>())
    }));
}