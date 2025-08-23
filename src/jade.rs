use std::path::PathBuf;

use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
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
}
#[derive(Deserialize, Serialize)]
pub struct Jade {
    pub config: Config,
    #[serde(skip)]
    pub song_current_selection: ListState,
    #[serde(skip)]
    pub queue_current_selection: ListState,
    #[serde(skip)]
    pub sound_increment: u8,
    #[serde(skip)]
    pub focus_area: FocusArea,
    #[serde(skip)]
    pub queue: Vec<String>,
    #[serde(skip)]
    pub songs: Songs,
    #[serde(skip)]
    pub channels: Channels,
    #[serde(skip)]
    pub current: Current,
}
impl Jade {
    pub fn change_focus_area(&mut self) {
        match self.focus_area {
            FocusArea::Music => self.focus_area = FocusArea::Queue,
            FocusArea::Queue => self.focus_area = FocusArea::Music,
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
pub struct Songs {
    pub titles: Vec<String>,
    pub lengths: Vec<u32>,
    pub visual_lengths: Vec<String>,
}

#[derive(Default)]
pub struct Current {
    pub title: String,
    pub length: u32,
    pub position: u32,
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
