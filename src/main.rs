mod app;
mod config;
mod keyhandling;
mod render;

use std::{env::home_dir, path::PathBuf};

use crate::{
    app::{app, state::AppState},
    config::Config,
};

pub const SUPPORTED_FORMATS: [&str; 4] = ["wav", "mp3", "ogg", "flac"];

fn main() -> color_eyre::Result<()> {
    // Get config path
    let config_location = if let Some(mut home_dir) = home_dir() {
        home_dir.push(".config/jade/config.toml");
        home_dir
    } else {
        panic!("Can't get home-dir from env");
    };

    // Load config from file, if not valid load as default
    let mut config = match Config::from_file(&config_location) {
        Ok(c) => c,
        Err(_) => Config::default(),
    };

    // If music location is invalid continually prompt user until valid
    let invalid_music_location = |s: &str| s.is_empty() || !PathBuf::from(s).exists();
    if config
        .music_location()
        .is_none_or(|ml| invalid_music_location(ml.as_str()))
    {
        loop {
            let mut music_location: String = String::new();
            println!("Music location invalid or empty\nEnter music location:",);
            std::io::stdin().read_line(&mut music_location)?;

            if !invalid_music_location(music_location.as_str().trim()) {
                config.set_music_location(music_location.trim().to_string());
                break;
            }
        }
    }

    let mut app_state = AppState::new(config);

    // Walk music location
    let _ = app_state.entries.explore(None);

    color_eyre::install()?;
    ratatui::run(|t| app(t, &mut app_state))?;

    app_state
        .config
        .save_to_file(config_location)
        .expect("Can't save config file");
    Ok(())
}
