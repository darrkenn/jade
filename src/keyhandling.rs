use crate::FocusArea::{Music as Music_Area, Queue as Queue_Area};
use crate::musicplayer::MusicPlayer::{End, NewSong, Pause, Stop, Volume};
use crate::queue::Queue;
use crate::{Jade, VOLUMELEVELS};
use crossterm::event;
use crossterm::event::{KeyEvent, KeyEventKind};
use std::fs;

pub fn handle_key(key: KeyEvent, jade: &mut Jade) -> bool {
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
            fs::write(jade.config.location.clone(), toml_data).expect("Cant write to file");
            jade.channels.s_mp.send(End).expect("Cant stop thread");
            return true;
        }
        //Audio controls
        event::KeyCode::Char('<') => {
            if jade.sound_increment > 0 {
                jade.sound_increment -= 1;
                jade.config.volume = VOLUMELEVELS[jade.sound_increment as usize];
                jade.channels
                    .s_mp
                    .send(Volume(jade.config.volume))
                    .expect("Couldnt decrease volume")
            }
        }
        event::KeyCode::Char('>') => {
            if jade.sound_increment < 10 {
                jade.sound_increment += 1;
                jade.config.volume = VOLUMELEVELS[jade.sound_increment as usize];
                jade.channels
                    .s_mp
                    .send(Volume(jade.config.volume))
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
                    let (song, length) = current_song(
                        jade.config.music_location.to_str().unwrap().to_string(),
                        &jade.songs.titles,
                        &jade.songs.lengths,
                        i,
                    );
                    jade.channels
                        .s_mp
                        .send(NewSong(song, length))
                        .expect("UhOh");
                }
            }
            event::KeyCode::Char('q') => {
                if let Some(i) = jade.song_current_selection.selected() {
                    let (song, length) = current_song(
                        jade.config.music_location.to_str().unwrap().to_string(),
                        &jade.songs.titles,
                        &jade.songs.lengths,
                        i,
                    );
                    jade.queue.push((song).parse().unwrap());
                    jade.channels
                        .s_q
                        .send(Queue::Add(song, length))
                        .expect("Cant send to queue");
                }
            }
            event::KeyCode::Char(' ') => {
                jade.channels.s_mp.send(Pause).expect("Couldnt pause song");
            }
            event::KeyCode::Backspace => {
                jade.channels.s_mp.send(Stop).expect("Couldnt stop song");
            }
            _ => {}
        }
    } else if jade.focus_area == Queue_Area {
        match key.code {
            event::KeyCode::Up => jade.queue_current_selection.select_previous(),
            event::KeyCode::Down => jade.queue_current_selection.select_next(),
            event::KeyCode::Char('d') => {
                let selection = jade.queue_current_selection.selected();
                if let Some(current_selection) = selection {
                    jade.queue.remove(current_selection);
                    jade.channels
                        .s_q
                        .send(Queue::Remove(current_selection))
                        .expect("Cant remove from queue");
                }
            }
            event::KeyCode::Backspace => jade
                .channels
                .s_q
                .send(Queue::Clear)
                .expect("Cant clear queue"),

            _ => {}
        }
    }
    false
}

fn current_song(location: String, songs: &[String], lengths: &[u32], i: usize) -> (String, u32) {
    if location.ends_with("/") {
        (format!("{location}{}", songs[i]), lengths[i])
    } else {
        (format!("{location}/{}", songs[i]), lengths[i])
    }
}
