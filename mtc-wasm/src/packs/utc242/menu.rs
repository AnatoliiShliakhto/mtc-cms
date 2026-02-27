use super::*;

#[component]
pub fn Menu() -> Element {
    let auth = state!(auth);

    rsx! {
        MenuItem {
            route: route!(),
            Icon { icon: Icons::Home, class: "size-8 sm:size-6 text-neutral" }
            { t!("menu-home") }
        }

        // 242 центр підготовки підрозділів
        li {
            details {
                summary {
                    Icon { icon: Icons::Info2, class: "size-8 sm:size-6 text-neutral" }
                    { t!("menu-main") }
                }
                ul {
                    MenuItem {
                        route: route!(API_CONTENT, "news"),
                        { t!("menu-news") }
                    }
                    MenuItem {
                        route: route!(API_CONTENT, "page", "history"),
                        { t!("menu-history") }
                    }
                    MenuItem {
                        route: route!(API_CONTENT, "vacancy"),
                        { t!("menu-vacancy") }
                    }
                    MenuItem {
                        route: route!(API_CONTENT, "page", "contacts"),
                        { t!("menu-contacts") }
                    }
                }
            }
        }

        // Програми підготовки
        if auth.has_permission("cadet::read") {
            li {
                details {
                    summary {
                        Icon { icon: Icons::Book, class: "size-8 sm:size-6 text-neutral" }
                        { t!("menu-m-programs") }
                    }
                    ul {
                        MenuItem {
                            route: route!(API_CONTENT, "page", "prg-individual"),
                            { t!("menu-prg-individual") }
                        }
                        MenuItem {
                            route: route!(API_CONTENT, "page", "prg-collective"),
                            { t!("menu-prg-collective") }
                        }
                    }
                }
            }
        }

        // Інструктору
        if auth.has_permission("instructor::read") {
            li {
                details {
                    summary {
                        Icon { icon: Icons::Medal, class: "size-8 sm:size-6 text-neutral" }
                        { t!("menu-m-instructor") }
                    }
                    ul {
                        li {
                            details {
                                summary {
                                    { t!("menu-in-sit") }
                                }
                                ul {
                                    MenuItem {
                                        route: route!(API_CONTENT, "course", "sit-bzvp"),
                                        { t!("menu-sit-bzvp") }
                                    }
                                    li {
                                        details {
                                            summary {
                                                { t!("menu-sit-pt") }
                                            }
                                            ul {
                                                MenuItem {
                                                    route: route!(API_CONTENT, API_COURSE, "sit-101-1"),
                                                    { t!("menu-sit-101-1") }
                                                }
                                                MenuItem {
                                                    route: route!(API_CONTENT, "course", "sit-101-2"),
                                                    { t!("menu-sit-101-2") }
                                                }
                                                MenuItem {
                                                    route: route!(API_CONTENT, "course", "sit-101-3"),
                                                    { t!("menu-sit-101-3") }
                                                }
                                                MenuItem {
                                                    route: route!(API_CONTENT, "course", "sit-102"),
                                                    { t!("menu-sit-102") }
                                                }
                                                MenuItem {
                                                    route: route!(API_CONTENT, "course", "sit-103"),
                                                    { t!("menu-sit-103") }
                                                }
                                            }
                                        }
                                    }
                                    MenuItem {
                                        route: route!(API_CONTENT, "course", "sit-sergeants"),
                                        { t!("menu-sit-sergeants") }
                                    }
                                }
                            }
                        }
                        li {
                            details {
                                summary {
                                    { t!("menu-sit-plan") }
                                }
                                ul {
                                    MenuItem {
                                        route: route!(API_CONTENT, "page", "sit-journal"),
                                        { t!("menu-sit-journal") }
                                    }
                                    MenuItem {
                                        route: route!(API_CONTENT, "page", "sit-schedule"),
                                        { t!("menu-sit-schedule") }
                                    }
                                }
                            }
                        }
                        MenuItem {
                            route: route!(API_CONTENT, "shooting_course"),
                            { t!("menu-shooting_course") }
                        }
                        MenuItem {
                            route: route!(API_CONTENT, "preparatory_exercises"),
                            { t!("menu-preparatory_exercises") }
                        }
                    }
                }
            }
        }

        // Курсанту
        li {
            details {
                summary {
                    Icon { icon: Icons::Man, class: "size-8 sm:size-6 text-neutral" }
                    { t!("menu-m-cadet") }
                }
                ul {
                    MenuItem {
                        route: route!(API_CONTENT, "page", "cad-day"),
                        permission: "cadet::read",
                        { t!("menu-cad-day") }
                    }
                    MenuItem {
                        route: route!(API_CONTENT, "page", "cad-laws"),
                        { t!("menu-cad-laws") }
                    }
                    MenuItem {
                        route: route!(API_CONTENT, "page", "cad-schedule"),
                        permission: "cadet::read",
                        { t!("menu-cad-schedule") }
                    }
                    li {
                        details {
                            summary {
                                { t!("menu-cad-self") }
                            }
                            ul {
                                MenuItem {
                                    route: route!(API_CONTENT, "page", "cad-bzvp"),
                                    { t!("menu-cad-bzvp") }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Методична робота
        if auth.has_permission("instructor::read") {
            li {
                details {
                    summary {
                        Icon { icon: Icons::Work, class: "size-8 sm:size-6 text-neutral" }
                        { t!("menu-m-work") }
                    }
                    ul {
                        MenuItem {
                            route: route!(API_CONTENT, "page", "mtd-ok"),
                            { t!("menu-mtd-ok") }
                        }
                        MenuItem {
                            route: route!(API_CONTENT, "page", "mtd-242"),
                            { t!("menu-mtd-242") }
                        }
                        MenuItem {
                            route: route!(API_CONTENT, "page", "mtd-exp"),
                            { t!("menu-mtd-exp") }
                        }
                        MenuItem {
                            route: route!(API_CONTENT, "page", "mtd-reports"),
                            { t!("menu-mtd-reports") }
                        }
                    }
                }
            }
        }

        // Навчальні матеріали
        li {
            details {
                summary {
                    Icon { icon: Icons::MortarBoard, class: "size-8 sm:size-6 text-neutral" }
                    { t!("menu-lm-root") }
                }
                ul {
                    li {
                        details {
                            summary { { t!("menu-lm-article") } }
                            ul {
                                MenuItem {
                                    route: route!(API_CONTENT, "lmatopography"),
                                    { t!("menu-lmatopography") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmaaction"),
                                    { t!("menu-lmaaction") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmabattle"),
                                    { t!("menu-lmabattle") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmaengineering"),
                                    { t!("menu-lmaengineering") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmalaws"),
                                    { t!("menu-lmalaws") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmaconnect"),
                                    { t!("menu-lmaconnect") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmaantiaircraft"),
                                    { t!("menu-lmaantiaircraft") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmarhbz"),
                                    { t!("menu-lmarhbz") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmamortar"),
                                    { t!("menu-lmamortar") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmaantitank"),
                                    { t!("menu-lmaantitank") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmalogistics"),
                                    { t!("menu-lmalogistics") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmapsychology"),
                                    { t!("menu-lmapsychology") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmasupport"),
                                    { t!("menu-lmasupport") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmarecon"),
                                    { t!("menu-lmarecon") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmamedicine"),
                                    { t!("menu-lmamedicine") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmatactics"),
                                    { t!("menu-lmatactics") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmaphysical"),
                                    { t!("menu-lmaphysical") }
                                }
                            }
                        }
                    }
                    li {
                        details {
                            summary { { t!("menu-lmlibrary") } }
                            ul {
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-battle"),
                                    { t!("menu-lml-battle") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-store"),
                                    { t!("menu-lml-store") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-bpla"),
                                    { t!("menu-lml-bpla") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-navy"),
                                    { t!("menu-lml-navy") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-safe"),
                                    { t!("menu-lml-safe") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-saving"),
                                    { t!("menu-lml-saving") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-topo"),
                                    { t!("menu-lml-topo") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-fire"),
                                    { t!("menu-lml-fire") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-dive"),
                                    { t!("menu-lml-dive") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-ref"),
                                    { t!("menu-lml-ref") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-mortar"),
                                    { t!("menu-lml-mortar") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-antiair"),
                                    { t!("menu-lml-antiair") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-logistics"),
                                    { t!("menu-lml-logistics") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-safety"),
                                    { t!("menu-lml-safety") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-engineering"),
                                    { t!("menu-lml-engineering") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-info"),
                                    { t!("menu-lml-info") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-psychosafety"),
                                    { t!("menu-lml-psychosafety") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-wlaw"),
                                    { t!("menu-lml-wlaw") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-national"),
                                    { t!("menu-lml-national") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-rhbz"),
                                    { t!("menu-lml-rhbz") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-actions"),
                                    { t!("menu-lml-actions") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-service"),
                                    { t!("menu-lml-service") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-connect"),
                                    { t!("menu-lml-connect") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-laws"),
                                    { t!("menu-lml-laws") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-staff"),
                                    { t!("menu-lml-staff") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-psycho"),
                                    { t!("menu-lml-psycho") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-food"),
                                    { t!("menu-lml-food") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-recon"),
                                    { t!("menu-lml-recon") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-special"),
                                    { t!("menu-lml-special") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-statutes"),
                                    { t!("menu-lml-statutes") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-military"),
                                    { t!("menu-lml-military") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-medicine"),
                                    { t!("menu-lml-medicine") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-tactics"),
                                    { t!("menu-lml-tactics") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-experience"),
                                    { t!("menu-lml-experience") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-phys"),
                                    { t!("menu-lml-phys") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmlibrary", "lml-civil"),
                                    { t!("menu-lml-civil") }
                                }
                            }
                        }
                    }
                    li {
                        details {
                            summary { { t!("menu-lmorcs") } }
                            ul {
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-topography"),
                                    { t!("menu-lmo-topography") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-action"),
                                    { t!("menu-lmo-action") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-battle"),
                                    { t!("menu-lmo-battle") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-bpla"),
                                    { t!("menu-lmo-bpla") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-engineering"),
                                    { t!("menu-lmo-engineering") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-connect"),
                                    { t!("menu-lmo-connect") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-antiaircraft"),
                                    { t!("menu-lmo-antiaircraft") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-rhbz"),
                                    { t!("menu-lmo-rhbz") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-mortar"),
                                    { t!("menu-lmo-mortar") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-antitank"),
                                    { t!("menu-lmo-antitank") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-logistics"),
                                    { t!("menu-lmo-logistics") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-psychology"),
                                    { t!("menu-lmo-psychology") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-support"),
                                    { t!("menu-lmo-support") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-recon"),
                                    { t!("menu-lmo-recon") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-medicine"),
                                    { t!("menu-lmo-medicine") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-tactics"),
                                    { t!("menu-lmo-tactics") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-physical"),
                                    { t!("menu-lmo-physical") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "lmorcs", "lmo-experience"),
                                    { t!("menu-lmo-experience") }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Повсякденна діяльність
        li {
            details {
                summary {
                    Icon { icon: Icons::JournalBookmark, class: "size-8 sm:size-6 text-neutral" }
                    { t!("menu-m-daily") }
                }
                ul {
                    if auth.has_permission("instructor::read") {
                        li {
                            details {
                                summary { { t!("menu-gd-docs") } }
                                ul {
                                    MenuItem {
                                        route: route!(API_CONTENT, "page", "gd-orders"),
                                        { t!("menu-gd-orders") }
                                    }
                                    MenuItem {
                                        route: route!(API_CONTENT, "page", "gd-directives"),
                                        { t!("menu-gd-directives") }
                                    }
                                    MenuItem {
                                        route: route!(API_CONTENT, "page", "gd-commands"),
                                        { t!("menu-gd-commands") }
                                    }
                                }
                            }
                        }
                    }
                    li {
                        details {
                            summary { { t!("menu-sd-docs") } }
                            ul {
                                MenuItem {
                                    route: route!(API_CONTENT, "page", "sd-reports"),
                                    { t!("menu-sd-reports") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "page", "sd-orders"),
                                    { t!("menu-sd-orders") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "page", "sd-data"),
                                    { t!("menu-sd-data") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "page", "sd-attachments"),
                                    { t!("menu-sd-attachments") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "page", "sd-commands"),
                                    { t!("menu-sd-commands") }
                                }
                                MenuItem {
                                    route: route!(API_CONTENT, "page", "sd-mps"),
                                    permission: "instructor::read",
                                    { t!("menu-sd-mps") }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}