#[derive(Debug)]
pub struct Memory {
    // 4096 Byte of RAM
    data : [u8; 4096]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data : [0; 4096]
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let mut result: u16 = self.read_byte(address) as u16;
        result = (result << 8) | (self.read_byte(address + 1) as u16);
        result
    }
}