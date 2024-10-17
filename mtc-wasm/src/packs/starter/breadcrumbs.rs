use crate::prelude::*;

pub fn build_breadcrumbs(slug: &str) {
    let slug: Cow<'static, str> = slug.to_owned().into();

    use_effect(use_reactive!(|(slug,)| {
        let breadcrumbs: Vec<(Cow<str>, &str)> = match &*slug {
            "menu-sign-in" => vec![(t!("menu-sign-in"), "/sign-in")],
            "menu-settings" => vec![(t!("menu-settings"), "/change-password")],
            "menu-search" => vec![(t!("menu-search"), "")],

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
                (t!("menu-page"), "/list/page")
            ],

            "menu-course" => vec![
                (t!("menu-content"), ""),
                (t!("menu-course"), "/list/course")
            ],

            "menu-content-edit" => vec![
                (t!("menu-content"), ""),
                (t!("menu-content-edit"), "")
            ],

            "menu-course-edit" => vec![
                (t!("menu-course"), ""),
                (t!("menu-course-edit"), "")
            ],

            _ => vec![],
        };

        use_breadcrumbs().set(breadcrumbs.into_iter()
            .map(|(key, value)| (key.into(), Cow::Borrowed(value.into())))
            .collect::<Vec<(Cow<'static, str>, Cow<'static, str>)>>())
    }));
}