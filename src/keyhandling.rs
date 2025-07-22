use std::fs;
use std::sync::mpsc::Sender;
use crossterm::event;
use crossterm::event::{KeyEvent, KeyEventKind};
use crate::{Jade, CONFIGFILE, VOLUMELEVELS};
use crate::musicplayer::MusicPlayer;
use crate::musicplayer::MusicPlayer::{NewSong, Pause, Stop, Volume};

pub fn handle_key(key:KeyEvent, jade: &mut Jade, songs: &Vec<String>, tx: Sender<MusicPlayer>) -> bool {
    if key.kind != KeyEventKind::Press {
        return false;
    }
    match key.code {
        event::KeyCode::Esc => {
            let toml_data = toml::to_string(&jade).unwrap();
            fs::write(CONFIGFILE, toml_data).expect("Cant write to file");
            return true
        }
        event::KeyCode::Up => {jade.current_selection.select_previous()}
        event::KeyCode::Down => {jade.current_selection.select_next()}
        //Music player commands
        event::KeyCode::Enter => {
            //Essential formatting for correct reading of song.

            if let Some(i) = jade.current_selection.selected() {
                let song_name =  &songs[i];
                    let song = if jade.music_location.ends_with("/") {
                        format!("{}{}", jade.music_location, song_name)
                    } else {
                        format!("{}/{}", jade.music_location, song_name)
                    };
                    tx.send(NewSong(song)).expect("UhOh");
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