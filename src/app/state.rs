use crate::{
    app::music::tree::{Node, NodeType},
    config::Config,
};

pub enum Screen {
    Songs,
    Queue,
    Settings,
}

pub struct AppState {
    pub config: Config,
    pub current_volume_level: f32,
    pub current_screen: Screen,
    pub entries: Option<Node>,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            current_volume_level: config.volume_level(),
            config,
            current_screen: Screen::Songs,
            entries: None,
        }
    }
    pub fn set_root_node(&mut self) {
        self.entries = Some(Node::new(
            self.config.music_location().unwrap(),
            None,
            NodeType::Folder,
        ))
    }
}
