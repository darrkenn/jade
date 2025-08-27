use std::path::PathBuf;

use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use edar::Metadata;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

use crate::threads::{
    info::Info,
    musicplayer::MusicPlayer,
    queue::{Queue, UpdateQueue},
};

#[derive(Deserialize, Serialize, Default, PartialEq, Debug)]
pub enum FocusArea {
    #[default]
    Music,
    Queue,
    Info,
}
#[derive(Deserialize, Serialize)]
pub struct App {
    pub config: Config,
    #[serde(skip)]
    pub current: Current,
    #[serde(skip)]
    pub sound_increment: u8,
    #[serde(skip)]
    pub focus_area: FocusArea,
    #[serde(skip)]
    pub queue: Vec<Song>,
    #[serde(skip)]
    pub songs: Vec<Song>,
    #[serde(skip)]
    pub channels: Channels,
    #[serde(skip)]
    pub song_info: Song,
}
impl App {
    pub fn change_focus_area(&mut self) {
        match self.focus_area {
            FocusArea::Music => self.focus_area = FocusArea::Queue,
            FocusArea::Queue => self.focus_area = FocusArea::Music,
            FocusArea::Info => self.focus_area = FocusArea::Info,
        }
    }
    pub fn change_focus_info(&mut self) {
        match self.focus_area {
            FocusArea::Info => self.focus_area = FocusArea::Music,
            _ => self.focus_area = FocusArea::Info,
        }
    }
}
#[derive(Deserialize, Serialize)]
pub struct Config {
    pub music_location: PathBuf,
    pub volume: f32,
    #[serde(skip)]
    pub location: PathBuf,
}

#[derive(Default)]
pub struct Current {
    pub selection: CurrentSelection,
    pub song: CurrentSong,
}

#[derive(Default)]
pub struct CurrentSelection {
    pub song: ListState,
    pub queue: ListState,
}

#[derive(Default)]
pub struct CurrentSong {
    pub song: Song,
    pub position: u32,
}

#[derive(Default, Clone)]
pub struct Song {
    pub metadata: Metadata,
    pub file_name: String,
}

pub struct Channels {
    pub s_mp: Sender<MusicPlayer>,
    pub r_mp: Receiver<MusicPlayer>,
    pub s_q: Sender<Queue>,
    pub r_update: Receiver<UpdateQueue>,
    pub r_ui: Receiver<Info>,
}
impl Default for Channels {
    fn default() -> Self {
        let (s_mp, r_mp) = unbounded::<MusicPlayer>();
        let (s_q, _) = unbounded::<Queue>();
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
