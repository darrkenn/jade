mod info;
mod keyhandling;
mod musicplayer;
mod queue;
mod render;
mod run;
mod song_information;

use crate::FocusArea::{Music, Queue};
use crate::info::{Info, create_info};
use crate::musicplayer::{MusicPlayer, Request, create_mp};
use crate::queue::{UpdateQueue, create_queue};
use crate::run::run;
use crate::song_information::get_songs_in_folder;
use color_eyre::eyre::Result;
use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::env::home_dir;
use std::fs;
use std::path::PathBuf;

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
    config: Config,
    #[serde(skip)]
    song_current_selection: ListState,
    #[serde(skip)]
    queue_current_selection: ListState,
    #[serde(skip)]
    sound_increment: u8,
    #[serde(skip)]
    focus_area: FocusArea,
    #[serde(skip)]
    queue: Vec<String>,
    #[serde(skip)]
    songs: Songs,
    #[serde(skip)]
    channels: Channels,
    #[serde(skip)]
    current: Current,
}
impl Jade {
    fn change_focus_area(&mut self) {
        match self.focus_area {
            Music => self.focus_area = Queue,
            Queue => self.focus_area = Music,
        }
    }
}
#[derive(Deserialize, Serialize)]
struct Config {
    music_location: PathBuf,
    volume: f32,
    #[serde(skip)]
    location: PathBuf,
}

#[derive(Default)]
struct Songs {
    titles: Vec<String>,
    lengths: Vec<u32>,
    visual_lengths: Vec<String>,
}

#[derive(Default)]
struct Current {
    title: String,
    length: u32,
    position: u32,
}

struct Channels {
    s_mp: Sender<MusicPlayer>,
    r_mp: Receiver<MusicPlayer>,
    s_q: Sender<queue::Queue>,
    r_update: Receiver<UpdateQueue>,
    r_ui: Receiver<Info>,
}
impl Default for Channels {
    fn default() -> Self {
        let (s_mp, r_mp) = unbounded::<MusicPlayer>();
        let (s_q, _) = unbounded::<queue::Queue>();
        let (_, r_update) = bounded::<UpdateQueue>(1);
        let (_, r_ui) = bounded::<Info>(2);
        Channels {
            s_mp,
            r_mp,
            s_q,
            r_update,
            r_ui,
        }
    }
}

fn main() -> Result<()> {
    //Config
    let config = get_config();
    let jade_string = fs::read_to_string(&config).expect("Cant find config file");
    let mut jade: Jade = toml::from_str((jade_string).as_ref()).expect("Cant parse file");
    //Setting values
    jade.config.location = config;
    jade.sound_increment = find_volume_location(jade.config.volume);
    (
        jade.songs.titles,
        jade.songs.lengths,
        jade.songs.visual_lengths,
    ) = get_songs_in_folder(jade.config.music_location.clone());
    jade.focus_area = Music;
    jade.song_current_selection.select_first();
    jade.queue_current_selection.select_first();

    // Thread creation
    let r_req: Receiver<Request>;
    let s_info: Sender<Info>;

    (s_info, jade.channels.r_ui) = create_info();

    (jade.channels.s_mp, jade.channels.r_mp, r_req) = create_mp(jade.config.volume, s_info);
    (jade.channels.s_q, jade.channels.r_update) = create_queue(jade.channels.s_mp.clone(), r_req);

    //Setup of UI
    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut jade);
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

fn get_config() -> PathBuf {
    if let Some(home) = home_dir() {
        let mut config_location = home.to_str().unwrap().to_owned();
        config_location.push_str("/.config/jade/config.toml");
        PathBuf::from(config_location)
    } else {
        panic!("Cant find config file");
    }
}
