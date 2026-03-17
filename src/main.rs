use std::{env::home_dir, fs};

use crate::{app::app, config::Config};

mod app;
mod config;
mod render;

pub const VOLUME_LEVELS: [f32; 11] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
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
    println!("{:#?}", config);
    color_eyre::install()?;
    ratatui::run(app)?;

    config
        .save_to_file(config_location)
        .expect("Can't save config file");
    Ok(())
}
