use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::state::{AppState, Screen},
    keyhandling::{
        queue::handle_queue_key, settings::handle_settings_key, songs::handle_songs_key,
    },
};

mod queue;
mod settings;
mod songs;

pub fn handle_key(key: KeyEvent, app_state: &mut AppState) -> bool {
    match key.code {
        KeyCode::Backspace => return true,
        KeyCode::Esc => app_state.current.screen = Screen::Settings,
        KeyCode::Char(c) => match c {
            's' => app_state.current.screen = Screen::Songs,
            'q' => app_state.current.screen = Screen::Queue,
            _ => {}
        },
        _ => {}
    };
    match app_state.current.screen {
        Screen::Songs => handle_songs_key(key, app_state),
        Screen::Queue => handle_queue_key(key, app_state),
        Screen::Settings => handle_settings_key(key, app_state),
    }
    false
}
