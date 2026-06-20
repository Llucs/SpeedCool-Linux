use crossterm::event::{KeyCode, KeyEvent};
use crate::core::engine::ProfileEngine;
use crate::core::profile::Profile;
use super::app::{App, AppScreen};

pub fn handle_key(app: &mut App, key: KeyEvent) {
    match app.screen {
        AppScreen::Main => handle_main_key(app, key),
        _ => handle_detail_key(app, key),
    }
}

fn handle_main_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
        KeyCode::Char('1') => app.screen = AppScreen::Cpu,
        KeyCode::Char('2') => app.screen = AppScreen::Memory,
        KeyCode::Char('3') => app.screen = AppScreen::Gpu,
        KeyCode::Char('4') => app.screen = AppScreen::Disk,
        KeyCode::Char('5') => app.screen = AppScreen::Thermal,
        KeyCode::Char('6') => app.screen = AppScreen::Battery,
        KeyCode::Char('e') | KeyCode::Char('E') => {
            let _ = ProfileEngine::apply(&Profile::Eco);
            app.current_profile = Profile::Eco;
        }
        KeyCode::Char('b') | KeyCode::Char('B') => {
            let _ = ProfileEngine::apply(&Profile::Balanced);
            app.current_profile = Profile::Balanced;
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            let _ = ProfileEngine::apply(&Profile::Performance);
            app.current_profile = Profile::Performance;
        }
        _ => {}
    }
}

fn handle_detail_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.screen = AppScreen::Main,
        _ => {}
    }
}
