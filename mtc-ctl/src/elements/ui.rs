use super::*;

pub fn draw_ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical(
        [
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ]).split(frame.area());

    draw_header(frame, app, chunks[0]);
    draw_menu(frame, app, chunks[1]);

    match app.tabs.index {
        0 => draw_system_tab(frame, app, chunks[2]),
        1 => draw_system_tab(frame, app, chunks[2]),
        _ => {}
    };
}