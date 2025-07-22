use std::fs;
use std::sync::mpsc::Sender;
use crossterm::event;
use crossterm::event::{KeyEvent, KeyEventKind};
use log::__private_api::loc;
use crate::{Jade, CONFIGFILE, VOLUMELEVELS};
use crate::musicplayer::MusicPlayer;
use crate::musicplayer::MusicPlayer::{AddToQueue, End, NewSong, Pause, Stop, Volume};

pub fn handle_key(key:KeyEvent, jade: &mut Jade, songs: &Vec<String>, tx: Sender<MusicPlayer>) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }
    match key.code {
        event::KeyCode::Esc => {
            let toml_data = toml::to_string(&jade).unwrap();
            fs::write(CONFIGFILE, toml_data).expect("Cant write to file");
            tx.send(End).expect("Cant stop thread");
            return true
        }
        event::KeyCode::Up => {jade.current_selection.select_previous()}
        event::KeyCode::Down => {jade.current_selection.select_next()}
        //Music player commands
        event::KeyCode::Enter => {
            //Essential formatting for correct reading of song.
            if let Some(i) = jade.current_selection.selected() {
                let song = current_song(jade.music_location.clone(), &songs, i);
                tx.send(NewSong(song)).expect("UhOh");
            }
        }
        event::KeyCode::Char('q') => {
            if let Some(i) = jade.current_selection.selected() {
                let song = current_song(jade.music_location.clone(), &songs, i);
                tx.send(AddToQueue(song)).expect("Cant add to queue");
            }
        }
        event::KeyCode::Char(' ') => {
            tx.send(Pause).expect("Couldnt pause song");
        }
        event::KeyCode::Backspace => {
            tx.send(Stop).expect("Couldnt stop song");
        }
        //Audio controls
        event::KeyCode::Char('<') => {
            if jade.sound_increment > 0 {
                jade.sound_increment -= 1;
                jade.volume = VOLUMELEVELS[jade.sound_increment as usize];
                tx.send(Volume(jade.volume)).expect("Couldnt decrease volume")
            }
        }
        event::KeyCode::Char('>') => {
            if jade.sound_increment < 10 {
                jade.sound_increment += 1;
                jade.volume = VOLUMELEVELS[jade.sound_increment as usize];
                tx.send(Volume(jade.volume)).expect("Couldnt increase volume")
            }
        }
        _ => {}
    }
    false
}

fn current_song(location: String, songs: &Vec<String>, i: usize) -> String {
        let song_name =  &songs[i];
        let song = if location.ends_with("/") {
            format!("{}{}", location, song_name)
        } else {
            format!("{}/{}", location, song_name)
        };
        song
}