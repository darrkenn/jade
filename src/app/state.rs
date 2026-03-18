use crate::config::Config;

pub struct AppState {
    pub config: Config,
    pub current_volume_level: f32,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            current_volume_level: config.volume_level(),
            config,
        }
    }
}
