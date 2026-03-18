use crate::config::Config;

pub enum Screen {
    Songs,
    Queue,
    Settings,
}

pub struct AppState {
    pub config: Config,
    pub current_volume_level: f32,
    pub current_screen: Screen,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            current_volume_level: config.volume_level(),
            config,
            current_screen: Screen::Songs,
        }
    }
}
