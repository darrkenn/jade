use crate::FocusArea::{Music as Music_Area, Queue as Queue_Area};
use crate::musicplayer::MusicPlayer::{End, NewSong, Pause, Stop, Volume};
use crate::musicplayer::{MusicPlayer, Queue};
use crate::{CONFIGFILE, Jade, VOLUMELEVELS};
use crossterm::event;
use crossterm::event::{KeyEvent, KeyEventKind};
use std::fs;
use std::sync::mpsc::Sender;

pub fn handle_key(
    key: KeyEvent,
    jade: &mut Jade,
    mp: Sender<MusicPlayer>,
    q: Sender<Queue>,
) -> bool {
    //Key filter
    if key.kind != KeyEventKind::Press {
        return false;
    }

    //Global keys
    match key.code {
        event::KeyCode::Tab => {
            jade.change_focus_area();
        }
        event::KeyCode::Esc => {
            let toml_data = toml::to_string(&jade).unwrap();
            fs::write(CONFIGFILE, toml_data).expect("Cant write to file");
            mp.send(End).expect("Cant stop thread");
            return true;
        }
        //Audio controls
        event::KeyCode::Char('<') => {
            if jade.sound_increment > 0 {
                jade.sound_increment -= 1;
                jade.volume = VOLUMELEVELS[jade.sound_increment as usize];
                mp.send(Volume(jade.volume))
                    .expect("Couldnt decrease volume")
            }
        }
        event::KeyCode::Char('>') => {
            if jade.sound_increment < 10 {
                jade.sound_increment += 1;
                jade.volume = VOLUMELEVELS[jade.sound_increment as usize];
                mp.send(Volume(jade.volume))
                    .expect("Couldnt increase volume")
            }
        }
        _ => {}
    }
    if jade.focus_area == Music_Area {
        match key.code {
            event::KeyCode::Up => jade.song_current_selection.select_previous(),
            event::KeyCode::Down => jade.song_current_selection.select_next(),
            //Music player commands
            event::KeyCode::Enter => {
                //Essential formatting for correct reading of song.
                if let Some(i) = jade.song_current_selection.selected() {
                    let song = current_song(jade.music_location.clone(), &jade.songs, i);
                    mp.send(NewSong(song)).expect("UhOh");
                }
            }
            event::KeyCode::Char('q') => {
                if let Some(i) = jade.song_current_selection.selected() {
                    let song = current_song(jade.music_location.clone(), &jade.songs, i);
                    jade.queue.push((song).parse().unwrap());
                    q.send(Queue::Add(song)).expect("Cant send to queue");
                }
            }
            event::KeyCode::Char(' ') => {
                mp.send(Pause).expect("Couldnt pause song");
            }
            event::KeyCode::Backspace => {
                mp.send(Stop).expect("Couldnt stop song");
            }
            _ => {}
        }
    } else if jade.focus_area == Queue_Area {
        match key.code {
            event::KeyCode::Up => jade.queue_current_selection.select_previous(),
            event::KeyCode::Down => jade.queue_current_selection.select_next(),
            event::KeyCode::Char('d') => {
                let selection = jade.queue_current_selection.selected();
                if !selection.is_none() {
                    let current_selection = selection.unwrap();
                    jade.queue.remove(current_selection);
                    q.send(Queue::Remove(selection.unwrap()))
                        .expect("Cant remove from queue");
                }
            }
            event::KeyCode::Backspace => q.send(Queue::Clear).expect("Cant clear queue"),

            _ => {}
        }
    }
    false
}

fn current_song(location: String, songs: &[String], i: usize) -> String {
    if location.ends_with("/") {
        format!("{location}{}", songs[i])
    } else {
        format!("{location}/{}", songs[i])
    }
}
