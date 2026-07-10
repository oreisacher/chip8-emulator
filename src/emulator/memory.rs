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
        if let Some(bye) = self.data.get_mut(address as usize) {
            *bye = value;
        }
    }

    pub fn read_byte(&self, address: u16) -> Option<u8> {
        self.data.get(address as usize).copied()
    }

    pub fn read_word(&self, address: u16) -> Option<u16> {
        let byte1 = self.read_byte(address)? as u16;
        let byte2 = self.read_byte(address + 1)? as u16;
        Some((byte1 << 8) | byte2)
    }
}