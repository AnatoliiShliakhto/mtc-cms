use prelude::*;

mod app;
mod style;
mod elements;
mod widgets;

pub mod prelude {
    pub use {
        super::{
            app::*,
            style::*,
            elements::prelude::*,
            widgets::*,
        },

        std::{
            error::Error,
            io,
            time::{Duration, Instant},
        },

        ratatui::{
            prelude::*,
            backend::{Backend, CrosstermBackend},
            crossterm::{
                event::{self, DisableMouseCapture, EnableMouseCapture,
                        Event, KeyCode, KeyEventKind, KeyModifiers, KeyEvent},
                execute,
                terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
            },
            Terminal,
            layout::{Constraint, Layout, Rect},
            style::{Color, Modifier, Style},
            symbols,
            text::{self, Span},
            widgets::{
                canvas::{self, Canvas, Circle, Map, MapResolution, Rectangle},
                Axis, BarChart, Block, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem, Paragraph,
                Row, Sparkline, Table, Tabs, Wrap,
            },
            Frame,
        }
    };
}

fn main() -> Result<(), Box<dyn Error>>  {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("MTC-CMS Controller");
    let app_result = run_app(&mut terminal, app, Duration::from_millis(400));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}
