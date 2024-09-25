use crate::prelude::*;

pub fn build_breadcrumbs(slug: &str) {
    let slug: Cow<'static, str> = slug.to_owned().into();

    use_effect(move || {
        let breadcrumbs: Vec<(Cow<str>, &str)> = match &*slug {
            "menu-sign-in" => vec![(t!("menu-sign-in"), "/sign-in")],
            "menu-settings" => vec![(t!("menu-settings"), "/change-password")],

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

            _ => vec![],
        };

        use_breadcrumbs().set(breadcrumbs.into_iter()
            .map(|(key, value)| (key.into(), Cow::Borrowed(value.into())))
            .collect::<BTreeMap<Cow<'static, str>, Cow<'static, str>>>())
    });
}

pub fn drop_breadcrumbs() {
    use_effect(move || {
        use_breadcrumbs().set(BTreeMap::new());
    });
}