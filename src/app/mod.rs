pub mod state;
use ratatui::DefaultTerminal;

use crate::{app::state::AppState, render::render};

pub fn app(terminal: &mut DefaultTerminal, app_state: &mut AppState) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}
