use crossterm::event::{KeyCode, KeyEvent};

use crate::app::state::AppState;

pub fn handle_songs_key(key: KeyEvent, app_state: &mut AppState) {
    match key.code {
        KeyCode::Up => app_state.current.index += 1,
        KeyCode::Down => app_state.current.index -= 1,
        _ => {}
    }
}
