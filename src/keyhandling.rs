use std::fmt::format;
use std::fs::{metadata, File};
use std::io::{BufReader, Seek};
use std::{fs, thread};
use std::time::Duration;
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
            let song_name = &songs[jade.current_selection.selected().unwrap()];

            let song = if jade.music_location.ends_with("/") {
                format!("{}{}", jade.music_location, song_name)
            } else {
                format!("{}/{}", jade.music_location, song_name)
            };
            let file = BufReader::new(File::open(&song).unwrap_or_else(|_| panic!("Cant read file: {}", song)));
            thread::spawn(move|| {
                let sink = rodio::play(&stream_handle.mixer(), file).unwrap();
                loop {
                    sink.play();
                }
            });
        }
        _ => {}
    }
    false
}