use super::*;

/// A component to view a list of links.
#[component]
pub fn ViewLinksField(
    value: Option<Value>,
) -> Element {
    if value.is_none() {
        return rsx! {}
    }

    let links = use_memo(use_reactive!(
        |value| value.self_obj::<Vec<LinkEntry>>().unwrap_or_default()
    ));

    rsx! {
        dl { class: "file-stack",
            for link in links.iter() {
                if link.url.is_empty() {
                    dt { span { { link.title.clone() } } }
                } else {
                    dd {
                        { LinkItem(link.title.clone(), link.url.clone()) }
                    }
                }
            }
        }
    }
}

pub fn LinkItem(title: Cow<'static, str>, url: Cow<'static, str>) -> Element {
    let extension = get_extension_from_filename(&url);

    if url.starts_with("/content") {
        return rsx! {
            Icon { icon: Icons::Description, class: "size-4" }
            Link {
                to: &*url,
                { title }
            }
        };
    }

    if extension.is_none() | url.starts_with("http") {
        return rsx! {
            Icon { icon: Icons::Link45Deg, class: "size-4 text-primary" }
            a {
                href: &*url,
                "onclick": "linkOpen(this); return false;",
                { title }
            }
        };
    }

    let extension = extension.unwrap_or_default();

    rsx! {
        match extension {
            "xls" | "xlsx" | "xlsm" => rsx! {
                Icon { icon: Icons::FileExcel, class: "size-4" }
            },
            "doc" | "docx" | "docm" => rsx! {
                Icon { icon: Icons::FileWord, class: "size-4" }
            },
            "pptx" | "pptm" => rsx! {
                Icon { icon: Icons::FilePowerPoint, class: "size-4" }
            },
            "pdf" => rsx! {
                Icon { icon: Icons::FilePdf, class: "size-4" }
            },
            _ => rsx! {
                Icon { icon: Icons::File, class: "size-4" }
            },
        }
        a {
            href: &*url,
            "onclick": r#"linkDownloadThenOpen(this); return false;"#,
            { title }
        }
    }
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}