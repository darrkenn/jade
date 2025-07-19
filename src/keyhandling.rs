use std::fmt::format;
use std::fs::File;
use std::io::BufReader;
use crossterm::event;
use crossterm::event::{KeyEvent, KeyEventKind};
use crate::Jade;

pub fn handle_key(key:KeyEvent, jade: &mut Jade, songs: Vec<String>) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }
    match key.code {
        event::KeyCode::Esc => {
            return true;
        }
        event::KeyCode::Up => {jade.current_selection.select_previous()}
        event::KeyCode::Down => {jade.current_selection.select_next()}
        event::KeyCode::Enter => {
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("Cant open stream");
            let song_name = &songs[0];

            let song = if jade.music_location.ends_with("/") {
                format!("{}{}", jade.music_location, song_name)
            } else {
                format!("{}/{}", jade.music_location, song_name)
            };

            let file = BufReader::new(File::open(&song).unwrap_or_else(|_| panic!("Cant read file: {}", song)));
            let sink = rodio::play(&stream_handle.mixer(), file).unwrap();
            sink.sleep_until_end()
        }
        _ => {}
    }
    false
}