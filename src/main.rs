mod run;
mod render;
mod keyhandling;
mod musicplayer;

use std::default::Default;
use std::{fs};
use std::fs::{File};
use std::path::{Path, PathBuf};
use color_eyre::eyre::Result;
use ratatui::widgets::{ListState};
use serde::{Deserialize, Serialize};
use symphonia::core::io::MediaSourceStream;
use symphonia::default::get_probe;
use crate::musicplayer::{create_mp};
use crate::run::run;

const CONFIGFILE: &str = "config.toml";
//There has to be a better way to do this.
const VOLUMELEVELS: [f32; 11] = [0.0,0.1,0.2,0.3,0.4,0.5,0.6,0.7,0.8,0.9,1.0];
const SUPPORTED_FORMATS: [&str; 4] = ["wav","mp3","ogg", "flac"];
#[derive(Deserialize,Serialize)]
struct Jade {
    music_location: String,
    volume: f32,
    #[serde(skip)]
    current_selection: ListState,
    #[serde(skip)]
    sound_increment: u8,
    #[serde(skip)]
    songs: Vec<String>,
    #[serde(skip)]
    lengths: Vec<u32>,
    #[serde(skip)]
    visual_lengths: Vec<String>
}

fn main() -> Result<()>{
    //Reading config
    let jade_string = fs::read_to_string(CONFIGFILE).expect("Cant find config file");
    let mut jade: Jade = toml::from_str((&jade_string).as_ref()).expect("Cant parse file");
    jade.sound_increment = find_volume_location(jade.volume);
    (jade.songs, jade.lengths, jade.visual_lengths) = get_songs_in_folder(jade.music_location.parse()?);

    if jade.songs.len() != 0 {
        jade.current_selection.select_first();
    }
    //Creating the music player sink thread
    let tx =  create_mp(jade.volume);

    //Setup of UI
    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut jade, tx);
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;

    result
}

fn get_songs_in_folder(music_folder: PathBuf) -> (Vec<String>, Vec<u32>, Vec<String>) {
    let mut songs = Vec::new();
    let mut files: Vec<File> = Vec::new();
    if let Ok(entries) = fs::read_dir(music_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                if SUPPORTED_FORMATS.contains(&entry.path().extension().unwrap().to_str().unwrap()) {
                    songs.push(entry.file_name().to_str().unwrap().to_string());
                    let file = &entry.path().to_string_lossy().to_string();
                    let path = Path::new(&file);
                    files.push(File::open(&path).expect("Cant open file"))
                }
            }
        }
    }

    let song_lengths = get_song_lengths(files);
    let visual_song_lengths = get_visual_lengths(song_lengths.clone());
    (songs, song_lengths, visual_song_lengths)
}

fn get_song_lengths(files: Vec<File>) -> Vec<u32> {
    let mut lengths: Vec<u32> = Vec::new();
    for file in  files {
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let probe = get_probe()
            .format(&Default::default(), mss, &Default::default(), &Default::default())
            .expect("Cant probe file");

        let format = probe.format;
        let track = format.default_track().expect("Cant find track");
        if let Some(sample_rate) = track.codec_params.sample_rate {
            if let Some(frames) = track.codec_params.n_frames {
                let length = frames as f64 / sample_rate as f64;
                lengths.push(length.round() as u32)
            }
        }
    }
    lengths
}

fn get_visual_lengths(lengths: Vec<u32>) -> Vec<String> {
    let mut visual_lengths: Vec<String> = Vec::new();
    for length in lengths.into_iter() {
        let mins = length / 60;
        let secs = length % 60;
        if secs < 10 {
            visual_lengths.push(format!("{}:0{}", mins,secs))
        } else {
            visual_lengths.push(format!("{}:{}", mins,secs))
        }
    }
    visual_lengths
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

