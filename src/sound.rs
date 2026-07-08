use rodio::{MixerDeviceSink, Player, Source};
use rodio::source::SineWave;

pub struct Sound {
    handle : MixerDeviceSink,
    player : Player,
}

impl Sound {
    pub fn new() -> Sound {
        let handle = rodio::DeviceSinkBuilder::open_default_sink().expect("Can't create audio sink");
        let player = Player::connect_new(&handle.mixer());

        Sound {
            handle,
            player
        }
    }

    pub fn play_sound(&mut self) {
        if (self.player.empty()) {
            self.player.append(
                SineWave::new(220.0).repeat_infinite().amplify(0.1)
            )
        }
    }

    pub fn stop_sound(&mut self) {
        self.player.stop();
    }

    pub fn playing(&mut self) -> bool {
        self.player.empty()
    }
}