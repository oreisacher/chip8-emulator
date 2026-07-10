use std::path::Path;
use std::time::Duration;
use rand::RngExt;
use crate::config::Config;
use crate::emulator::quirks::Quirks;
use super::framebuffer::Framebuffer;
use super::keypad::Keypad;
use super::memory::Memory;

const PROGRAM_START : u16 = 0x200;

const FONTSET_START : u16 = 0x050;

const FONTSET : [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    pub loaded_rom : String,

    // --- RAM
    mem : Memory,

    // --- Registers
    // 16 8-bit registers labeled V0 - VF
    v : [u8; 16],

    // 16-bit register labeled I
    i : u16,

    // --- Timers
    delay_timer : u8,
    pub sound_timer : u8,

    // Program Counter
    pc : u16,

    // Stack
    sp : u8,
    stack : [u16; 16],

    // --- Keypad
    keypad : Keypad,

    // --- Framebuffer
    pub fb : Framebuffer,

    // --- Quirks
    quirks : Quirks
}

impl Chip8 {
    pub fn new(config : &Config) -> Chip8 {
        let mut mem = Memory::new();

        // Load font set into memory
        for i in 0..FONTSET.len() {
            mem.write_byte(FONTSET_START + i as u16, FONTSET[i]);
        }

        Chip8 {
            loaded_rom : "".to_string(),
            mem,
            v : [0; 16],
            i : 0,
            delay_timer : 0,
            sound_timer : 0,
            pc : PROGRAM_START,
            sp : 0,
            stack : [0; 16],
            fb : Framebuffer::new(),
            keypad: Keypad::new(),
            quirks : Quirks::new(config)
        }
    }

    pub fn load_rom(&mut self, rom_path : String) {
        println!("Loading ROM: {}", rom_path);

        let rom : Vec<u8> = std::fs::read(&rom_path).expect("Could not load ROM");

        for i in 0..rom.len() {
            self.mem.write_byte(PROGRAM_START + i as u16, rom[i]);
        }

        // Extract name of loaded rom
        self.loaded_rom = Path::new(&rom_path).file_stem().unwrap().to_str().unwrap().to_string();

        println!("Loaded {} bytes from ROM '{}'", rom.len(), self.loaded_rom);
    }

    pub fn cycle(&mut self) {
        // Fetch
        let Some(opcode) = self.mem.read_word(self.pc) else {
            println!("Invalid memory address.");
            self.pc += 2;
            return;
        };

        self.pc += 2;

        // Decode & Execute instruction
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00e0 => self.op_00e0(),
                    0x00ee => self.op_00ee(),
                    _ => println!("Unrecognized instruction 0x{:X}", opcode)
                }
            },
            0x1000 => self.op_1nnn(opcode),
            0x2000 => self.op_2nnn(opcode),
            0x3000 => self.op_3xkk(opcode),
            0x4000 => self.op_4xkk(opcode),
            0x5000 => self.op_5xy0(opcode),
            0x6000 => self.op_6xkk(opcode),
            0x7000 => self.op_7xkk(opcode),
            0x8000 => {
                match opcode & 0x000F{
                    0x0 => self.op_8xy0(opcode),
                    0x1 => self.op_8xy1(opcode),
                    0x2 => self.op_8xy2(opcode),
                    0x3 => self.op_8xy3(opcode),
                    0x4 => self.op_8xy4(opcode),
                    0x5 => self.op_8xy5(opcode),
                    0x6 => self.op_8xy6(opcode),
                    0x7 => self.op_8xy7(opcode),
                    0xe => self.op_8xye(opcode),
                    _ => println!("Unrecognized instruction 0x{:X}", opcode)
                }
            }
            0x9000 => self.op_9xy0(opcode),
            0xA000 => self.op_annn(opcode),
            0xB000 => self.op_bnnn(opcode),
            0xC000 => self.op_cxkk(opcode),
            0xD000 => self.op_dxyn(opcode),
            0xE000 => {
                match opcode & 0x00FF {
                    0x9E => self.op_ex9e(opcode),
                    0xA1 => self.op_exa1(opcode),
                    _ => println!("Unrecognized instruction 0x{:X}", opcode)
                }
            }
            0xF000 => {
                match opcode & 0x00FF {
                    0x07 => self.op_fx07(opcode),
                    0x0A => self.op_fx0a(opcode),
                    0x15 => self.op_fx15(opcode),
                    0x18 => self.op_fx18(opcode),
                    0x1E => self.op_fx1e(opcode),
                    0x29 => self.op_fx29(opcode),
                    0x33 => self.op_fx33(opcode),
                    0x55 => self.op_fx55(opcode),
                    0x65 => self.op_fx65(opcode),
                    _ => println!("Unrecognized instruction 0x{:X}", opcode)
                }
            }
            _ => println!("Unrecognized instruction 0x{:X}", opcode)
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    // --- Keyboard ---
    pub fn update_keyboard_states(&mut self) {
        self.keypad.tick();
    }

    pub fn press_key(&mut self, key : u8) {
        self.keypad.press_key(key);
    }

    pub fn release_key(&mut self, key : u8) {
        self.keypad.release_key(key);
    }

    // --- Helpers ---
    fn get_x(opcode : u16) -> usize { ((opcode & 0x0F00) >> 8) as usize }

    fn get_y(opcode : u16) -> usize { ((opcode & 0x00F0) >> 4) as usize }

    fn get_n(opcode : u16) -> usize { ((opcode & 0x000F) >> 0) as usize }

    fn get_nnn(opcode : u16) -> u16 { opcode & 0x0FFF }

    fn get_kk(opcode : u16) -> u8 { (opcode & 0x00FF) as u8 }

    // --- Operations ---

    /// Clear the display
    fn op_00e0(&mut self) {
        self.fb.clear();
    }

    /// Return from a subroutine
    fn op_00ee(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    /// Jump to location nnn
    fn op_1nnn(&mut self, opcode : u16) {
        self.pc = Self::get_nnn(opcode);
    }

    /// Call subroutine at nnn
    fn op_2nnn(&mut self, opcode : u16) {
        let nnn  = Self::get_nnn(opcode);

        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn;
    }

    /// Skip next instruction if Vx = kk
    fn op_3xkk(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let kk = Self::get_kk(opcode);

        if self.v[x] == kk {
            self.pc += 2;
        }
    }

    /// Skip next instruction if Vx != kk
    fn op_4xkk(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let kk = Self::get_kk(opcode);

        if self.v[x] != kk {
            self.pc += 2;
        }
    }

    /// Skip next instruction if Vx = Vy
    fn op_5xy0(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    /// Set Vx = kk
    fn op_6xkk(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let kk = Self::get_kk(opcode);

        self.v[x] = kk;
    }

    /// Set Vx = Vx + kk
    fn op_7xkk(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let kk = Self::get_kk(opcode);

        self.v[x] = self.v[x].wrapping_add(kk);
    }

    /// Set Vx = Vy
    fn op_8xy0(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        self.v[x] = self.v[y];
    }

    /// Set Vx = Vx | Vy
    fn op_8xy1(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        self.v[x] |= self.v[y];

        if self.quirks.vf_reset {
            self.v[0xF] = 0;
        }
    }

    /// Set Vx = Vx & Vy
    fn op_8xy2(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        self.v[x] &= self.v[y];

        if self.quirks.vf_reset {
            self.v[0xF] = 0;
        }
    }

    /// Set Vx = Vx ^ Vy
    fn op_8xy3(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        self.v[x] ^= self.v[y];

        if self.quirks.vf_reset {
            self.v[0xF] = 0;
        }
    }

    /// Set Vx = Vx + Vy, set VF = carry
    fn op_8xy4(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        let (result, carry) = self.v[x].overflowing_add(self.v[y]);

        self.v[x] = result;
        self.v[0xF] = if carry { 1 } else { 0 };
    }

    /// Set Vx = Vx - Vy, set VF = NOT borrow
    fn op_8xy5(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);

        self.v[x] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    /// Set Vx = Vx SHR 1
    fn op_8xy6(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        if self.quirks.shifting {
            self.v[x] = self.v[y];
        }

        let carry = self.v[x] & 0x0001 == 1;
        self.v[x] /= 2;
        self.v[0xF] = if carry { 1 } else { 0 };
    }

    /// Set Vx = Vy - Vx, set VF = NOT borrow
    fn op_8xy7(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);

        self.v[x] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }

    /// Set Vx = Vx SHL 1
    fn op_8xye(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        if self.quirks.shifting {
            self.v[x] = self.v[y];
        }

        let carry = (self.v[x] & 0x80) != 0;
        self.v[x] = self.v[x].wrapping_mul(2);
        self.v[0xF] = if carry { 1 } else { 0 };
    }

    /// Skip next instruction if Vx != Vy
    fn op_9xy0(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    /// Set I = nnn
    fn op_annn(&mut self, opcode : u16) {
        self.i = Self::get_nnn(opcode);
    }

    /// Jump to location nnn + V0
    fn op_bnnn(&mut self, opcode : u16) {
        let nnn  = Self::get_nnn(opcode);

        self.pc = nnn + self.v[0] as u16;
    }

    /// Set Vx = random byte AND kk
    fn op_cxkk(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);
        let kk = Self::get_kk(opcode);

        let rnd_value = rand::rng().random_range(0..=255) as u8;
        self.v[x] = rnd_value & kk;
    }

    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision
    fn op_dxyn(&mut self, opcode : u16) {
        let n = Self::get_n(opcode);
        let x = Self::get_x(opcode);
        let y = Self::get_y(opcode);

        self.v[0xF] = 0;

        let mut pos_x = self.v[x] % 64;
        let mut pos_y = self.v[y] % 32;

        for i in 0..n {
            let index = self.i + i as u16;
            let Some(mut byte) = self.mem.read_byte(index) else {
                break;
            };

            byte = byte.reverse_bits();

            for b in 0..8 {
                let bit = (byte >> b) & 1;

                let mut px = pos_x + b;

                if self.quirks.clipping {
                    if px > 63 {
                        break;
                    }
                } else {
                    px = px % 64;
                }

                if self.fb.xor(px, pos_y, bit) {
                    self.v[0xF] = 1;
                }
            }

            pos_y = pos_y + 1;

            if self.quirks.clipping {
                if pos_y > 31 {
                    break;
                }
            } else {
                pos_y = pos_y % 32;
            }
        }
    }

    /// Skip next instruction if key with the value of Vx is pressed
    fn op_ex9e(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        if self.keypad.is_key_down(self.v[x]) {
            self.pc += 2;
        }
    }

    /// Skip next instruction if key with the value of Vx is not pressed
    fn op_exa1(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        if !self.keypad.is_key_down(self.v[x]) {
            self.pc += 2;
        }
    }

    /// Set Vx = delay timer value
    fn op_fx07(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        self.v[x] = self.delay_timer;
    }

    /// Wait for a key press, store the value of the key in Vx
    fn op_fx0a(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        match self.keypad.just_released() {
            Some(key) => self.v[x] = key,

            // Reduce program counter by 2 so the same instruction is run again
            None => self.pc -= 2,
        }
    }

    /// Set delay timer = Vx
    fn op_fx15(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        self.delay_timer = self.v[x];
    }

    /// Set sound timer = Vx
    fn op_fx18(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        self.sound_timer = self.v[x];
    }

    /// Set I = I + Vx
    fn op_fx1e(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        self.i += self.v[x] as u16;
    }

    /// Set I = location of sprite for digit Vx
    fn op_fx29(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        self.i = FONTSET_START + self.v[x] as u16;
    }

    /// Store BCD representation of Vx in memory locations I, I+1, and I+2
    fn op_fx33(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        let value = self.v[x];
        self.mem.write_byte(self.i, value / 100);
        self.mem.write_byte(self.i + 1, (value / 10) % 10);
        self.mem.write_byte(self.i + 2, value % 10);
    }

    /// Store registers V0 through Vx in memory starting at location I
    fn op_fx55(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        for i in 0..=x {
            self.mem.write_byte(self.i + i as u16, self.v[i]);
        }

        if self.quirks.memory_increment {
            self.i = self.i + x as u16 + 1;
        }
    }

    /// Read registers V0 through Vx from memory starting at location I
    fn op_fx65(&mut self, opcode : u16) {
        let x = Self::get_x(opcode);

        for i in 0..=x {
            if let Some(byte) = self.mem.read_byte(self.i + i as u16) {
                self.v[i] = byte;
            }
        }

        if self.quirks.memory_increment {
            self.i = self.i + x as u16 + 1;
        }
    }
}