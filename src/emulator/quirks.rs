use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Serialize, Deserialize)]
pub struct Quirks {
    pub vf_reset: bool,
    pub memory_increment : bool,
    pub clipping : bool,
    pub shifting : bool
}

impl Quirks {
    pub fn new(config: &Config) -> Quirks {
        Quirks {
            vf_reset : config.quirks.vf_reset,
            memory_increment : config.quirks.memory_increment,
            clipping : config.quirks.clipping,
            shifting : config.quirks.shifting,
        }
    }
}

impl Default for Quirks {
    fn default() -> Quirks {
        Quirks {
            vf_reset : false,
            memory_increment : true,
            clipping : false,
            shifting : false,
        }
    }
}