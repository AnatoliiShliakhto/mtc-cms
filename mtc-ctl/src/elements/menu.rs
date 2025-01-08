use ratatui::widgets::{BorderType, Borders};
use super::*;

pub fn draw_menu(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::horizontal(
        vec![Constraint::Length(50), Constraint::Min(0)]
    ).margin(1).split(area);

    let border = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
    frame.render_widget(border, area);

    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, app.style.tab)))
        .collect::<Tabs>()
        .highlight_style(app.style.tab_active)
        .select(app.tabs.index);
    frame.render_widget(tabs, chunks[0]);

    let text: Vec<Line> = vec![
        text::Line::from(vec![
            Span::styled("<F1>", app.style.hotkey),
            Span::styled(" System ", app.style.subtitle),
            Span::styled("| ", app.style.input),
            Span::styled("<F2>", app.style.hotkey),
            Span::styled(" Logs ", app.style.subtitle),
            Span::styled("| ", app.style.input),
            Span::styled("<Ctrl+C>", app.style.hotkey),
            Span::styled(" Exit ", app.style.subtitle),
        ])
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default())
        .alignment(Alignment::Right);
    frame.render_widget(paragraph, chunks[1]);
}