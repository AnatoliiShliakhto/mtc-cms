use super::*;

pub fn ThemeSwitcher() -> Element {
    let mut dark_theme =
        use_local_storage("dark_theme", || true);
    
    let switch_theme = move |_| {
        dark_theme.set(!dark_theme.get())
    };

    rsx! {
        div { class: "btn btn-ghost join-item",
            prevent_default: "onclick",
            onclick: switch_theme,
            label { class: "swap swap-rotate",
                input {
                    value: "light",
                    r#type: "checkbox",
                    class: "theme-controller",
                    checked: dark_theme.get(),
                }
                Icon { icon: Icons::Sun, class: "size-8 sm:size-6 fill-current swap-off" }
                Icon { icon: Icons::Moon, class: "size-8 sm:size-6 fill-current swap-on" }
            }
        }
    }
}