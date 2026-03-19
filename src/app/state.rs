use std::{cell::RefCell, path::PathBuf, rc::Rc};

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
    pub list: Vec<Node>,
    pub list_size: usize,
    pub index: usize,
}

pub struct AppState {
    pub config: Config,
    pub current: Current,
    pub root: Rc<RefCell<Node>>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            current: Current {
                volume_level: config.volume_level(),
                screen: Screen::Songs,
                list: Vec::new(),
                list_size: 0,
                index: 0,
            },
            root: Rc::new(RefCell::new(Node::new(
                config.music_location().unwrap(),
                None,
                NodeType::Folder,
                None,
            ))),
            config,
        }
    }
}
