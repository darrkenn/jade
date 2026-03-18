pub mod music;
pub mod state;
use crossterm::event::Event;
use ratatui::DefaultTerminal;

use crate::{app::state::AppState, keyhandling::handle_key, render::render};

pub fn app(terminal: &mut DefaultTerminal, app_state: &mut AppState) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| render(f, app_state))?;
        if let Event::Key(key) = crossterm::event::read()? {
            if key.is_press() {
                if handle_key(key, app_state) {
                    break Ok(());
                }
            }
        }
    }
}
