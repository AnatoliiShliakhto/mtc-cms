use ratatui::crossterm::style::Stylize;

use super::*;

pub fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let title = Paragraph::new(Span::styled(
        app.title,
        app.style.title,
    ))
        .block(Block::default())
        .alignment(Alignment::Center);
    f.render_widget(title, area);

    let text = format!("v{} ", env!("CARGO_PKG_VERSION"),);

    let meta = Paragraph::new(Span::styled(text, app.style.subtitle))
        .block(Block::default())
        .alignment(Alignment::Right);
    f.render_widget(meta, area);
}