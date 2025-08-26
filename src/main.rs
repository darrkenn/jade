mod app;
mod keyhandling;
mod render;
mod run;
mod song_information;
mod threads;

use crate::app::FocusArea::Music as Music_Area;
use crate::app::{App, Song};
use crate::run::run;
use crate::threads::info::{Info, create_info};
use crate::threads::musicplayer::{MusicPlayer, Request, create_mp};
use crate::threads::queue::create_queue;
use color_eyre::eyre::Result;
use crossbeam_channel::{Receiver, Sender};
use edar::{Extractor, Metadata};
use std::env::home_dir;
use std::path::PathBuf;
use std::{fs, process};

const VOLUMELEVELS: [f32; 11] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
const SUPPORTED_FORMATS: [&str; 4] = ["wav", "mp3", "ogg", "flac"];

fn main() -> Result<()> {
    //Config
    let config = get_config();
    let app_string = fs::read_to_string(&config).expect("Cant find config file");
    let mut app: App = toml::from_str((app_string).as_ref()).expect("Cant parse file");

    let app = setup_app(&mut app, config);

    //Setup of UI
    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let _ = run(terminal, app);
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;
    process::exit(0);
}

fn find_volume_location(jade_volume: f32) -> u8 {
    let mut volume_location: u8 = 0;
    for (i, &volume) in VOLUMELEVELS.iter().enumerate() {
        if volume == jade_volume {
            volume_location = i as u8;
            break;
        }
    }
    volume_location
}

fn get_config() -> PathBuf {
    if let Some(home) = home_dir() {
        let mut config_location = home.to_str().unwrap().to_owned();
        config_location.push_str("/.config/jade/config.toml");
        PathBuf::from(config_location)
    } else {
        panic!("Cant find config file");
    }
}

fn setup_app(app: &mut App, config: PathBuf) -> &mut App {
    app.config.location = config;
    app.sound_increment = find_volume_location(app.config.volume);

    app.songs = get_songs_in_folder(app.config.music_location.clone());
    app.focus_area = Music_Area;
    app.song_current_selection.select_first();
    app.queue_current_selection.select_first();

    let r_req: Receiver<Request>;
    let s_info: Sender<Info>;

    (s_info, app.channels.r_ui) = create_info();

    (app.channels.s_mp, app.channels.r_mp, r_req) =
        create_mp(app.config.volume, s_info, app.config.music_location.clone());
    (app.channels.s_q, app.channels.r_update) = create_queue(app.channels.s_mp.clone(), r_req);

    app
}

fn get_songs_in_folder(music_folder: PathBuf) -> Vec<Song> {
    let mut songs: Vec<Song> = Vec::new();

    if let Ok(entries) = fs::read_dir(music_folder) {
        for entry in entries.flatten() {
            if let Some(extension) = entry.path().extension().and_then(|e| e.to_str()) {
                if SUPPORTED_FORMATS.contains(&extension) {
                    if let Ok(metadata) = Extractor::extract_metadata(
                        entry.path().to_str().expect("Cant convert path to str"),
                    ) {
                        let song = Song {
                            metadata: metadata,
                            file_name: entry.file_name().to_str().unwrap().to_string(),
                        };
                        songs.push(song);
                    } else {
                        let song = Song {
                            metadata: Metadata::default(),
                            file_name: entry.file_name().to_str().unwrap().to_string(),
                        };
                        songs.push(song);
                    }
                }
            }
        }
    };
    songs
}
