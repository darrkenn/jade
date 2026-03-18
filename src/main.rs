mod app;
mod config;
mod render;

use std::env::home_dir;

use crate::{
    app::{app, state::AppState},
    config::Config,
};

pub const SUPPORTED_FORMATS: [&str; 4] = ["wav", "mp3", "ogg", "flac"];

fn main() -> color_eyre::Result<()> {
    let config_location = if let Some(mut home_dir) = home_dir() {
        home_dir.push(".config/jade/config.toml");
        home_dir
    } else {
        panic!("Can't get home-dir from env");
    };

    let config = match Config::from_file(&config_location) {
        Ok(c) => c,
        Err(_) => Config::default(),
    };

    let mut app_state = AppState::new(config);

    color_eyre::install()?;
    ratatui::run(|t| app(t, &mut app_state))?;

    app_state
        .config
        .save_to_file(config_location)
        .expect("Can't save config file");
    Ok(())
}
