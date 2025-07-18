mod run;
mod render;
mod keyhandling;

use std::fs;
use std::path::PathBuf;
use color_eyre::eyre::Result;
use ratatui::widgets::{List, ListItem};
use serde::{Deserialize, Serialize};
use toml::value::Array;
use crate::run::run;

const CONFIGFILE: &str = "config.toml";

#[derive(Deserialize,Serialize)]
struct JadeConfig {
    music_location: String,
    volume: u8
}

fn main(){
    //Reading config
    let jade_config_string = fs::read_to_string(CONFIGFILE).expect("Cant find config file");
    let mut jade_config: JadeConfig = toml::from_str((&jade_config_string).as_ref()).expect("Cant parse file");



    let songs = get_songs_in_folder(jade_config.music_location.parse().unwrap());
    for i in 0..songs.len() {
        println!("{}", songs[i])
    }
    //Setup of UI
    // color_eyre::install()?;
    // crossterm::terminal::enable_raw_mode()?;
    // let terminal = ratatui::init();
    // let result = run(terminal, &mut jade_config);
    // ratatui::restore();
    // crossterm::terminal::disable_raw_mode()?;
    //
    // result
}

fn get_songs_in_folder(music_folder: PathBuf) -> Vec<String>{
    let mut songs = Vec::new();
    if let Ok(entries) = fs::read_dir(music_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                songs.push(entry.file_name().to_str().unwrap().to_string())
            }
        }
    }
    songs
}
