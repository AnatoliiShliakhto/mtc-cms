use crate::prelude::*;

#[derive(Debug)]
pub struct AppStyle {
    pub title: Style,
    pub subtitle: Style,
    pub hotkey: Style,
    pub text: Style,
    pub selected: Style,
    pub input: Style,
    pub accent: Style,
    pub success: Style,
    pub error: Style,
    pub warning: Style,
    pub tab: Style,
    pub tab_active: Style,
}

impl Default for AppStyle {
    fn default() -> Self {
        Self {
            title: Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
            subtitle: Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::ITALIC),
            hotkey: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            text: Style::default().fg(Color::White),
            selected: Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            input: Style::default().add_modifier(Modifier::ITALIC),
            accent: Style::default().fg(Color::Cyan),
            success: Style::default().fg(Color::Green),
            error: Style::default().fg(Color::Red),
            warning: Style::default().fg(Color::Yellow),
            tab: Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD),
            tab_active: Style::default().fg(Color::LightYellow).add_modifier(Modifier::BOLD),
        }
    }
}