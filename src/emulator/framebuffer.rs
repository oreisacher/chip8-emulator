#[derive(Debug)]
pub struct Framebuffer {
    // Pixel data of 64x32 display
    pub pixels : [u8; 64 * 32]
}

impl Framebuffer {
    pub fn new() -> Framebuffer {
        Framebuffer {
            pixels : [0; 64 * 32]
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [0; 64 * 32]
    }

    pub fn xor(&mut self, x : u8, y : u8, value: u8) -> bool {
        let index = y as u16 * 64 + (x as u16);

        let prev_on = self.pixels[index as usize] == 1;
        self.pixels[index as usize] ^= value;

        prev_on && self.pixels[index as usize] == 0
    }
}