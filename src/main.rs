mod jade;
mod keyhandling;
mod render;
mod run;
mod song_information;
mod threads;

use crate::jade::FocusArea::Music as Music_Area;
use crate::jade::Jade;
use crate::run::run;
use crate::threads::info::{Info, create_info};
use crate::threads::musicplayer::{MusicPlayer, Request, create_mp};
use crate::threads::queue::create_queue;
use color_eyre::eyre::Result;
use crossbeam_channel::{Receiver, Sender};
use song_information::get_song_info::get_songs_in_folder;
use std::env::home_dir;
use std::path::PathBuf;
use std::{fs, process};

const VOLUMELEVELS: [f32; 11] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
const SUPPORTED_FORMATS: [&str; 4] = ["wav", "mp3", "ogg", "flac"];

fn main() -> Result<()> {
    //Config
    let config = get_config();
    let jade_string = fs::read_to_string(&config).expect("Cant find config file");
    let mut jade: Jade = toml::from_str((jade_string).as_ref()).expect("Cant parse file");

    let jade = setup_jade(&mut jade, config);

    //Setup of UI
    color_eyre::install()?;
    crossterm::terminal::enable_raw_mode()?;
    let terminal = ratatui::init();
    let _ = run(terminal, jade);
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

fn setup_jade(jade: &mut Jade, config: PathBuf) -> &mut Jade {
    jade.config.location = config;
    jade.sound_increment = find_volume_location(jade.config.volume);

    (
        jade.songs.titles,
        jade.songs.lengths,
        jade.songs.visual_lengths,
    ) = get_songs_in_folder(jade.config.music_location.clone());

    jade.focus_area = Music_Area;
    jade.song_current_selection.select_first();
    jade.queue_current_selection.select_first();

    let r_req: Receiver<Request>;
    let s_info: Sender<Info>;

    (s_info, jade.channels.r_ui) = create_info();

    (jade.channels.s_mp, jade.channels.r_mp, r_req) = create_mp(jade.config.volume, s_info);
    (jade.channels.s_q, jade.channels.r_update) = create_queue(jade.channels.s_mp.clone(), r_req);

    jade
}
