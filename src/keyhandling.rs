use std::sync::mpsc::Sender;
use crossterm::event;
use crossterm::event::{KeyEvent, KeyEventKind};
use crate::Jade;
use crate::musicplayer::MusicPlayer;
use crate::musicplayer::MusicPlayer::{NewSong, Pause};

pub fn handle_key(key:KeyEvent, jade: &mut Jade, songs: &Vec<String>, tx: Sender<MusicPlayer>) -> bool {
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
            let song_name = &songs[jade.current_selection.selected().unwrap()];
            let song = if jade.music_location.ends_with("/") {
                format!("{}{}", jade.music_location, song_name)
            } else {
                format!("{}/{}", jade.music_location, song_name)
            };
            tx.send(NewSong(song)).expect("UhOh");
        }
        event::KeyCode::Char(' ') => {
            tx.send(Pause).expect("UhOh");
        }
        _ => {}
    }
    false
}