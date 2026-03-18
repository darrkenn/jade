use std::{env::home_dir, fs, path::PathBuf};

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

use crate::VOLUME_LEVELS;

pub struct Theme {
    text: Color,
    border: Color,
    background: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            text: Color::White,
            border: Color::White,
            background: Color::Black,
        }
    }
}

impl Theme {
    fn blue() -> Self {
        Self {
            text: Color::Blue,
            border: Color::Blue,
            background: Color::Black,
        }
    }
    fn red() -> Self {
        Self {
            text: Color::Red,
            border: Color::Red,
            background: Color::Black,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SelectedTheme {
    Default,
    Blue,
    Red,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    music_location: Option<String>,
    volume_level: Option<usize>,
    selected_theme: Option<SelectedTheme>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            music_location: None,
            volume_level: Some(1),
            selected_theme: Some(SelectedTheme::Default),
        }
    }
}

impl Config {
    pub fn save_to_file(&self, config_location: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        if !config_location.exists() {
            if let Some(parent_folder) = config_location.parent() {
                fs::create_dir_all(parent_folder)?;
            }
            fs::File::create(&config_location)?;
        }
        fs::write(config_location, toml::to_string(&self)?)?;
        Ok(())
    }
    pub fn from_file(config_location: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(toml::from_str(&fs::read_to_string(&config_location)?)?)
    }

    pub fn music_location(&self) -> Option<String> {
        self.music_location.clone()
    }
    // Only use if music_location is Some
    pub fn music_location_as_pathbuf(&self) -> PathBuf {
        PathBuf::from(self.music_location.clone().unwrap())
    }
    pub fn set_music_location(&mut self, selected_location: String) {
        self.music_location = Some(selected_location)
    }

    pub fn theme(&self) -> &Theme {
        match self
            .selected_theme
            .as_ref()
            .unwrap_or(&SelectedTheme::Default)
        {
            SelectedTheme::Default => &Theme {
                text: Color::White,
                border: Color::White,
                background: Color::Black,
            },
            SelectedTheme::Blue => &Theme {
                text: Color::Blue,
                border: Color::Blue,
                background: Color::Black,
            },
            SelectedTheme::Red => &Theme {
                text: Color::Red,
                border: Color::Red,
                background: Color::Black,
            },
        }
    }

    pub fn volume_level(&self) -> f32 {
        if self.volume_level.is_none_or(|vl| vl > 100) {
            1.0
        } else {
            (self.volume_level.unwrap() / 100) as f32
        }
    }

    pub fn set_volume_level(&mut self, current_volume_level: f32) {
        let volume_level = (current_volume_level * 100_f32).round() as usize;
        self.volume_level = if volume_level <= 100 {
            Some(volume_level)
        } else {
            Some(100)
        }
    }
}
