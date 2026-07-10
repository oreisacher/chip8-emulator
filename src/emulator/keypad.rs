pub struct Keypad {
    current : u16,
    previous : u16
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad { current : 0, previous : 0 }
    }

    pub fn press_key(&mut self, key: u8) {
        self.current |= 1 << key;
    }

    pub fn release_key(&mut self, key: u8) {
        self.current &= !(1 << key);
    }

    pub fn is_key_down(&self, key: u8) -> bool {
        self.current & (1 << key) != 0
    }

    pub fn tick(&mut self) {
        self.previous = self.current;
    }

    pub fn just_released(&self) -> Option<u8> {
        let released = self.previous & !self.current;

        if released == 0 {
            None
        } else {
            Some(released.trailing_ones() as u8)
        }
    }
}