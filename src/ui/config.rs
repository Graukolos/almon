use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Config {
    pub video_mode: VideoMode,
    pub resolution: (u32, u32),
}

impl Config {
    pub fn load(path: String) -> Self {
        match File::open(path.as_str()) {
            Ok(file) => match serde_yaml::from_reader(file) {
                Ok(config) => config,
                Err(_) => Self::default(),
            },
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self, path: String) {
        if let Ok(file) = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path.as_str()) { if serde_yaml::to_writer(file, self).is_ok() {} }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            video_mode: VideoMode::Fullscreen,
            resolution: (1280, 720),
        }
    }
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum VideoMode {
    Windowed,
    Fullscreen,
}
