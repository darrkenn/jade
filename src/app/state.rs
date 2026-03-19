use std::path::PathBuf;

use crate::{
    app::music::tree::{Node, NodeType},
    config::Config,
};

pub enum Screen {
    Songs,
    Queue,
    Settings,
}

pub struct Current {
    pub volume_level: f32,
    pub screen: Screen,
    pub list: Vec<(String, String)>,
    pub index: usize,
}

pub struct AppState {
    pub config: Config,
    pub current: Current,
    pub entries: Node,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            current: Current {
                volume_level: config.volume_level(),
                screen: Screen::Songs,
                list: Vec::new(),
                index: 0,
            },
            entries: Node::new(config.music_location().unwrap(), None, NodeType::Folder),
            config,
        }
    }
}
