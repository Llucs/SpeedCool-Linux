pub mod app;
pub mod ui;
pub mod handlers;

use app::App;
use ui::draw;
use handlers::handle_key;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use std::time::Duration;

pub fn run_tui() -> Result<(), String> {
    enable_raw_mode().map_err(|e| e.to_string())?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).map_err(|e| e.to_string())?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| e.to_string())?;
    terminal.clear().map_err(|e| e.to_string())?;

    let mut app = App::new();
    let tick_rate = Duration::from_secs(2);

    let res = loop {
        terminal.draw(|f| draw(f, &app)).map_err(|e| e.to_string())?;

        if app.should_quit {
            break Ok(());
        }

        if app.last_poll.elapsed() >= app.poll_interval {
            app.poll();
        }

        if event::poll(tick_rate).map_err(|e| e.to_string())? {
            if let Event::Key(key) = event::read().map_err(|e| e.to_string())? {
                handle_key(&mut app, key);
            }
        }
    };

    disable_raw_mode().map_err(|e| e.to_string())?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture).map_err(|e| e.to_string())?;
    terminal.show_cursor().map_err(|e| e.to_string())?;

    res
}
