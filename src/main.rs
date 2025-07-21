mod run;
mod render;
mod keyhandling;

use std::{fs, thread};
use std::sync::mpsc;
use std::path::PathBuf;
use color_eyre::eyre::Result;
use ratatui::widgets::{ListState};
use serde::{Deserialize, Serialize};
use crate::run::run;

enum MusicPlayer {
    Pause,
    Play,
    Stop,
    CurrentPosition(),
}

const CONFIGFILE: &str = "config.toml";

#[derive(Deserialize,Serialize)]
struct Jade {
    music_location: String,
    volume: u8,
    #[serde(skip)]
    current_selection: ListState,
}

fn main() -> Result<()>{
    //Reading config
    let jade_string = fs::read_to_string(CONFIGFILE).expect("Cant find config file");
    let mut jade: Jade = toml::from_str((&jade_string).as_ref()).expect("Cant parse file");
    let songs = get_songs_in_folder(jade.music_location.parse()?);
    let (tx, rx) = mpsc::channel::<String>();

    let val = String::from("hi");
    tx.send(val)?;
    thread::spawn(move || {
        let val = rx.recv().unwrap();
        panic!("TEST TEST {}",val)
    });

    //Setup of UI
    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut jade, songs);
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
