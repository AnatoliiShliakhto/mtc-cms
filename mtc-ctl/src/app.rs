use crate::prelude::*;

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub style: AppStyle,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        Self {
            title,
            tabs: TabsState::new(vec!["System Information", "Log Viewer"]),
            style: AppStyle::default(),
        }
    }

    pub fn tab(&mut self, index: usize) {
        self.tabs.index = index
    }

    pub fn on_tick(&self) {

    }
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub const fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    'worker: loop {
        terminal.draw(|frame| draw_ui(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key {
                    KeyEvent {
                        kind: KeyEventKind::Press,
                        code: KeyCode::Char('c'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => break 'worker,
                    KeyEvent {
                        kind: KeyEventKind::Press,
                        code: KeyCode::F(1),
                        ..
                    } => app.tab(0),
                    KeyEvent {
                        kind: KeyEventKind::Press,
                        code: KeyCode::F(2),
                        ..
                    } => app.tab(1),
                    _ => (),
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    Ok(())
}