use crate::prelude::*;

pub fn build_breadcrumbs(slug: &str) {
    let slug: Cow<'static, str> = slug.to_owned().into();

    use_effect(use_reactive!(|(slug,)| {
        let breadcrumbs: Vec<(String, &str)> = match &*slug {
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

            "menu-news" => vec![
                (t!("menu-news"), "/content/news"),
            ],
            "menu-vacancy" => vec![
                (t!("menu-vacancy"), "/content/vacancy"),
            ],
            "menu-history" => vec![
                (t!("menu-history"), ""),
            ],
            "menu-contacts" => vec![
                (t!("menu-contacts"), ""),
            ],
            "menu-app-download"
            | "menu-app-data" => vec![
                (t!("menu-application"), ""),
            ],

            "menu-gate-passes" => vec![
                (t!("menu-administrator"), "/administrator"),
                (t!("menu-gate-passes"), "/administrator/gate-passes")
            ],

            "menu-gate-pass-validation-scan" => vec![
                (t!("menu-gate-pass-validation-scan"), "")
            ],

            "menu-gate-pass-validation-scan-result" => vec![
                (t!("menu-gate-pass-validation-scan"), "/gate-pass-validation-scans"),
                (t!("menu-gate-pass-validation-scan-result"), "")
            ],

            //Програми підготовки
            "menu-prg-individual"
            | "menu-prg-collective" => vec![
                (t!("menu-m-programs"), ""),
            ],

            // Інструктору
            "menu-sit-bzvp" => vec![
                (t!("menu-m-instructor"), ""),
                (t!("menu-in-sit"), ""),
            ],
            "menu-sit-101-1"
            | "menu-sit-101-2"
            | "menu-sit-101-3"
            | "menu-sit-102"
            | "menu-sit-103"
            => vec![
                (t!("menu-m-instructor"), ""),
                (t!("menu-in-sit"), ""),
                (t!("menu-sit-pt"), ""),
            ],
            "menu-sit-sergeants" => vec![
                (t!("menu-m-instructor"), ""),
                (t!("menu-in-sit"), ""),
            ],
            "menu-sit-journal"
            | "menu-sit-schedule" => vec![
                (t!("menu-m-instructor"), ""),
                (t!("menu-sit-plan"), ""),
            ],
            "menu-shooting_course" => vec![
                (t!("menu-m-instructor"), ""),
                (t!("menu-shooting_course"), "/content/shooting_course"),
            ],
            "menu-preparatory_exercises" => vec![
                (t!("menu-m-instructor"), ""),
                (t!("menu-preparatory_exercises"), "/content/preparatory_exercises"),
            ],

            // Курсанту
            "menu-cad-day"
            | "menu-cad-laws"
            | "menu-cad-schedule" => vec![
                (t!("menu-m-cadet"), ""),
            ],
            "menu-cad-bzvp" => vec![
                (t!("menu-m-cadet"), ""),
                (t!("menu-cad-self"), ""),
            ],

            // Методична робота
            "menu-mtd-ok"
            | "menu-mtd-242"
            | "menu-mtd-exp"
            | "menu-mtd-reports" => vec![
                (t!("menu-m-work"), ""),
            ],

            // Навчальні матеріали
            "menu-lmatopography"
            | "menu-lmaaction"
            | "menu-lmabattle"
            | "menu-lmaengineering"
            | "menu-lmalaws"
            | "menu-lmaconnect"
            | "menu-lmaantiaircraft"
            | "menu-lmarhbz"
            | "menu-lmamortar"
            | "menu-lmaantitank"
            | "menu-lmalogistics"
            | "menu-lmapsychology"
            | "menu-lmasupport"
            | "menu-lmarecon"
            | "menu-lmamedicine"
            | "menu-lmatactics"
            |"menu-lmaphysical" => vec![
                (t!("menu-lm-root"), ""),
                (t!("menu-lm-article"), ""),
            ],

            // Військові публікації
            "menu-lmlibrary" => vec![
                (t!("menu-lm-root"), ""),
                (t!("menu-lmlibrary"), "/content/lmlibrary"),
            ],

            // Військові публікації ПІДАРІВ
            "menu-lmorcs" => vec![
                (t!("menu-lm-root"), ""),
                (t!("menu-lmorcs"), "/content/lmorcs"),
            ],

            //Повсякденна діяльність
            "menu-gd-orders"
            | "menu-gd-directives"
            | "menu-gd-commands" => vec![
                (t!("menu-m-daily"), ""),
                (t!("menu-gd-docs"), ""),
            ],

            "menu-sd-reports"
            | "menu-sd-orders"
            | "menu-sd-data"
            | "menu-sd-attachments"
            | "menu-sd-commands"
            | "menu-sd-mps" => vec![
                (t!("menu-m-daily"), ""),
                (t!("menu-sd-docs"), ""),
            ],

            _ => vec![],
        };

        state!(set_breadcrumbs, breadcrumbs.into_iter()
            .map(|(key, value)| (key.into(), Cow::Borrowed(value.into())))
            .collect::<Vec<(Cow<'static, str>, Cow<'static, str>)>>())
    }));
}