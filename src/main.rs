mod keyhandling;
mod musicplayer;
mod render;
mod run;
mod song_information;

use crate::FocusArea::{Music, Queue};
use crate::musicplayer::create_mp;
use crate::run::run;
use crate::song_information::get_songs_in_folder;
use color_eyre::eyre::Result;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::fs;

const CONFIGFILE: &str = "config.toml";
//There has to be a better way to do this.
const VOLUMELEVELS: [f32; 11] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
const SUPPORTED_FORMATS: [&str; 4] = ["wav", "mp3", "ogg", "flac"];

#[derive(Deserialize, Serialize, Default, PartialEq, Debug)]
enum FocusArea {
    #[default]
    Music,
    Queue,
}
#[derive(Deserialize, Serialize)]
struct Jade {
    music_location: String,
    volume: f32,
    #[serde(skip)]
    song_current_selection: ListState,
    #[serde(skip)]
    queue_current_selection: ListState,
    #[serde(skip)]
    sound_increment: u8,
    #[serde(skip)]
    songs: Vec<String>,
    #[serde(skip)]
    lengths: Vec<u32>,
    #[serde(skip)]
    visual_lengths: Vec<String>,
    #[serde(skip)]
    focus_area: FocusArea,
    #[serde(skip)]
    queue: Vec<String>,
    #[serde(skip)]
    current_position: u64,
}

impl Jade {
    fn change_focus_area(&mut self) {
        match self.focus_area {
            Music => self.focus_area = Queue,
            Queue => self.focus_area = Music,
        }
    }
}

fn main() -> Result<()> {
    //Reading config
    let jade_string = fs::read_to_string(CONFIGFILE).expect("Cant find config file");
    let mut jade: Jade = toml::from_str((jade_string).as_ref()).expect("Cant parse file");
    //Setting values
    jade.sound_increment = find_volume_location(jade.volume);
    (jade.songs, jade.lengths, jade.visual_lengths) =
        get_songs_in_folder(jade.music_location.parse()?);
    jade.focus_area = Music;
    jade.song_current_selection.select_first();
    jade.queue_current_selection.select_first();

    // Thread creation
    let (mp, q) = create_mp(jade.volume);

    //Setup of UI
    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut jade, mp, q);
    ratatui::restore();
    crossterm::terminal::disable_raw_mode()?;

    result
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
