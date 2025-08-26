use crate::app::FocusArea::{Music as Music_Area, Queue as Queue_Area};
use crate::threads::musicplayer::MusicPlayer::{NewSong, Pause, Stop, Volume};
use crate::threads::queue::Queue::*;
use crate::{App, VOLUMELEVELS};
use crossterm::event;
use crossterm::event::{KeyEvent, KeyEventKind};
use std::fs;

pub fn handle_key(key: KeyEvent, app: &mut App) -> bool {
    //Key filter
    if key.kind != KeyEventKind::Press {
        return false;
    }

    //Global keys
    match key.code {
        event::KeyCode::Tab => {
            app.change_focus_area();
        }
        event::KeyCode::Esc => {
            let toml_data = toml::to_string(&app).unwrap();
            fs::write(app.config.location.clone(), toml_data).expect("Cant write to file");
            return true;
        }
        //Audio controls
        event::KeyCode::Char('<') => {
            if app.sound_increment > 0 {
                app.sound_increment -= 1;
                app.config.volume = VOLUMELEVELS[app.sound_increment as usize];
                app.channels
                    .s_mp
                    .send(Volume(app.config.volume))
                    .expect("Couldnt decrease volume")
            }
        }
        event::KeyCode::Char('>') => {
            if app.sound_increment < 10 {
                app.sound_increment += 1;
                app.config.volume = VOLUMELEVELS[app.sound_increment as usize];
                app.channels
                    .s_mp
                    .send(Volume(app.config.volume))
                    .expect("Couldnt increase volume")
            }
        }
        _ => {}
    }
    if app.focus_area == Music_Area {
        match key.code {
            event::KeyCode::Up => app.song_current_selection.select_previous(),
            event::KeyCode::Down => app.song_current_selection.select_next(),
            //Music player commands
            event::KeyCode::Enter => {
                //Essential formatting for correct reading of song.
                if let Some(i) = app.song_current_selection.selected() {
                    let song = &app.songs[i];
                    app.channels.s_mp.send(NewSong(song.clone())).expect("UhOh");
                }
            }
            event::KeyCode::Char('q') => {
                if let Some(i) = app.song_current_selection.selected() {
                    let song = &app.songs[i];
                    app.queue.push(song.clone());
                    app.channels
                        .s_q
                        .send(Add(song.clone()))
                        .expect("Cant send to queue");
                }
            }
            event::KeyCode::Char(' ') => {
                app.channels.s_mp.send(Pause).expect("Couldnt pause song");
            }
            event::KeyCode::Backspace => {
                app.channels.s_mp.send(Stop).expect("Couldnt stop song");
            }
            _ => {}
        }
    } else if app.focus_area == Queue_Area {
        match key.code {
            event::KeyCode::Up => app.queue_current_selection.select_previous(),
            event::KeyCode::Down => app.queue_current_selection.select_next(),
            event::KeyCode::Char('d') => {
                let selection = app.queue_current_selection.selected();
                if let Some(current_selection) = selection {
                    app.queue.remove(current_selection);
                    app.channels
                        .s_q
                        .send(Remove(current_selection))
                        .expect("Cant remove from queue");
                }
            }
            event::KeyCode::Backspace => app.channels.s_q.send(Clear).expect("Cant clear queue"),

            _ => {}
        }
    }
    false
}
