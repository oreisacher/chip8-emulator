use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Serialize, Deserialize)]
pub struct DisplaySettings {
    pub on_color : [f32; 3],
    pub off_color : [f32; 3],
}

impl DisplaySettings {
    pub fn new(config: &Config) -> DisplaySettings {
        DisplaySettings {
            on_color : config.display.on_color,
            off_color : config.display.off_color,
        }
    }
}

impl Default for DisplaySettings {
    fn default() -> DisplaySettings {
        DisplaySettings {
            on_color : [1.0, 1.0, 1.0],
            off_color : [0.0, 0.0, 0.0],
        }
    }
}