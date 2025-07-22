mod run;
mod render;
mod keyhandling;
mod musicplayer;

use std::{fs};
use std::path::PathBuf;
use color_eyre::eyre::Result;
use ratatui::widgets::{List, ListState};
use serde::{Deserialize, Serialize};
use crate::musicplayer::create_mp;
use crate::run::run;

const CONFIGFILE: &str = "config.toml";

//There has to be a better way to do this.
const VOLUMELEVELS: [f32; 11] = [0.0,0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9,1.0];

#[derive(Deserialize,Serialize)]
struct Jade {
    music_location: String,
    volume: f32,
    #[serde(skip)]
    current_selection: ListState,
    #[serde(skip)]
    sound_increment: u8
}

fn main() -> Result<()>{
    //Reading config
    let jade_string = fs::read_to_string(CONFIGFILE).expect("Cant find config file");
    let mut jade: Jade = toml::from_str((&jade_string).as_ref()).expect("Cant parse file");
    jade.sound_increment = find_volume_location(jade.volume);
    let songs = get_songs_in_folder(jade.music_location.parse()?);
    if songs.len() != 0 {
        jade.current_selection.select_first();
    }
    //Creating the music player sink thread
    let tx =  create_mp(jade.volume);
    
    //Setup of UI
    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut jade, songs, tx);
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;

    result
}

fn get_songs_in_folder(music_folder: PathBuf) -> Vec<String>{
    let mut songs = Vec::new();
    let supported_formats = ["wav","mp3","ogg", "flac"];
    if let Ok(entries) = fs::read_dir(music_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                if supported_formats.contains(&entry.path().extension().unwrap().to_str().unwrap()) {
                    songs.push(entry.file_name().to_str().unwrap().to_string())
                }
            }
        }
    }
    songs
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
