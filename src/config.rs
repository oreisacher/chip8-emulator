use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::emulator::quirks::Quirks;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub cpu_hz: u32,
    pub quirks: Quirks,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cpu_hz: 700,
            quirks: Quirks::default(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Config {
        if Path::new(path).exists() {
            let data = fs::read_to_string(path).expect("Failed to read config.");
            toml::from_str(&data).expect("Invalid config.")
        } else {
            println!("No config file found. Creating default config.");

            let config = Self::default();
            let data = toml::to_string_pretty(&config).unwrap();
            fs::write(path, data).expect("Failed to create config.");
            config
        }
    }
}