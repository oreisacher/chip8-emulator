use std::time::Duration;
use rand::RngExt;
use crate::framebuffer::Framebuffer;
use crate::memory::Memory;
use crate::sound::Sound;

pub struct Chip8 {
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

    pub timer_interval : Duration,

    // Program Counter
    pc : u16,

    // Stack
    sp : u8,
    stack : [u16; 16],

    // --- Keypad
    keypad : [KeyState; 16],

    // --- Framebuffer
    pub fb : Framebuffer,

    // --- Quirks
    quirks : Quirks
}

struct Quirks {
    vf_reset: bool,
    memory_increment : bool,
    clipping : bool,
    shifting : bool
}

impl Quirks {
    fn new() -> Quirks {
        Quirks {
            vf_reset : false,
            memory_increment : false,
            clipping : false,
            shifting : false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct KeyState {
    prev_state : bool,
    curr_state : bool,
}

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

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut mem = Memory::new();

        // Load font set into memory
        for i in 0..FONTSET.len() {
            mem.write_byte(FONTSET_START + i as u16, FONTSET[i]);
        }

        Chip8 {
            mem,
            v : [0; 16],
            i : 0,
            delay_timer : 0,
            sound_timer : 0,
            timer_interval : Duration::from_secs_f64(1.0 / 60.0),
            pc : PROGRAM_START,
            sp : 0,
            stack : [0; 16],
            fb : Framebuffer::new(),
            keypad: [KeyState { prev_state: false, curr_state : false }; 16],
            quirks : Quirks::new()
        }
    }

    pub fn load_rom(&mut self, rom_path : String) {
        println!("Loading ROM: {}", rom_path);

        let rom : Vec<u8> = std::fs::read(rom_path).expect("Could not load ROM");

        for i in 0..rom.len() {
            self.mem.write_byte(PROGRAM_START + i as u16, rom[i]);
        }

        println!("Loaded {} bytes from ROM", rom.len());
    }

    pub fn cycle(&mut self) {
        // Fetch
        let instruction : u16 = self.mem.read_word(self.pc);
        self.pc += 2;

        // Decode & Execute
        match instruction & 0xF000 {
            0x0000 => {
                match instruction {
                    0x00e0 => self.op_00e0(instruction),
                    0x00ee => self.op_00ee(instruction),
                    _ => println!("Unrecognized instruction 0x{:X}", instruction)
                }
            },
            0x1000 => self.op_1nnn(instruction),
            0x2000 => self.op_2nnn(instruction),
            0x3000 => self.op_3xkk(instruction),
            0x4000 => self.op_4xkk(instruction),
            0x5000 => self.op_5xy0(instruction),
            0x6000 => self.op_6xkk(instruction),
            0x7000 => self.op_7xkk(instruction),
            0x8000 => {
                match instruction & 0x000F{
                    0x0 => self.op_8xy0(instruction),
                    0x1 => self.op_8xy1(instruction),
                    0x2 => self.op_8xy2(instruction),
                    0x3 => self.op_8xy3(instruction),
                    0x4 => self.op_8xy4(instruction),
                    0x5 => self.op_8xy5(instruction),
                    0x6 => self.op_8xy6(instruction),
                    0x7 => self.op_8xy7(instruction),
                    0xe => self.op_8xye(instruction),
                    _ => println!("Unrecognized instruction 0x{:X}", instruction)
                }
            }
            0x9000 => self.op_9xy0(instruction),
            0xA000 => self.op_annn(instruction),
            0xB000 => self.op_bnnn(instruction),
            0xC000 => self.op_cxkk(instruction),
            0xD000 => self.op_dxyn(instruction),
            0xE000 => {
                match instruction & 0x00FF {
                    0x9E => self.op_ex9e(instruction),
                    0xA1 => self.op_exa1(instruction),
                    _ => println!("Unrecognized instruction 0x{:X}", instruction)
                }
            }
            0xF000 => {
                match instruction & 0x00FF {
                    0x07 => self.op_fx07(instruction),
                    0x0A => self.op_fx0a(instruction),
                    0x15 => self.op_fx15(instruction),
                    0x18 => self.op_fx18(instruction),
                    0x1E => self.op_fx1e(instruction),
                    0x29 => self.op_fx29(instruction),
                    0x33 => self.op_fx33(instruction),
                    0x55 => self.op_fx55(instruction),
                    0x65 => self.op_fx65(instruction),
                    _ => println!("Unrecognized instruction 0x{:X}", instruction)
                }
            }
            _ => println!("Unrecognized instruction 0x{:X}", instruction)
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

    pub fn update_keyboard_states(&mut self) {
        for i in self.keypad.iter_mut() {
            i.prev_state = i.curr_state;
        }
    }

    pub fn playing_sound(&self) -> bool {
        self.sound_timer > 0
    }

    pub fn press_key(&mut self, key : u8) {
        self.keypad[key as usize].curr_state = true;
    }

    pub fn release_key(&mut self, key : u8) {
        self.keypad[key as usize].curr_state = false;
    }

    // --- Operations
    fn op_00e0(&mut self, opcode : u16) {
        self.fb.clear();
    }
    fn op_00ee(&mut self, opcode : u16) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }
    fn op_1nnn(&mut self, opcode : u16) {
        self.pc = opcode & 0x0FFF;
    }
    fn op_2nnn(&mut self, opcode : u16) {
        let nnn  = opcode & 0x0FFF;

        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn;
    }
    fn op_3xkk(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;

        if self.v[x as usize] == kk {
            self.pc += 2;
        }
    }
    fn op_4xkk(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;

        if self.v[x as usize] != kk {
            self.pc += 2;
        }
    }
    fn op_5xy0(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        if (self.v[x as usize] == self.v[y as usize]) {
            self.pc += 2;
        }
    }
    fn op_6xkk(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;

        self.v[x as usize] = kk;
    }
    fn op_7xkk(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;

        self.v[x as usize] = self.v[x as usize].wrapping_add(kk);
    }
    fn op_8xy0(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        self.v[x as usize] = self.v[y as usize];
    }
    fn op_8xy1(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.v[x] |= self.v[y];

        if self.quirks.vf_reset {
            self.v[0xF] = 0;
        }
    }
    fn op_8xy2(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.v[x] &= self.v[y];

        if self.quirks.vf_reset {
            self.v[0xF] = 0;
        }
    }
    fn op_8xy3(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.v[x] ^= self.v[y];

        if self.quirks.vf_reset {
            self.v[0xF] = 0;
        }
    }
    fn op_8xy4(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let (result, carry) = self.v[x].overflowing_add(self.v[y]);

        self.v[x] = result;

        if carry {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
    }
    fn op_8xy5(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);

        self.v[x] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }
    fn op_8xy6(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        if self.quirks.shifting {
            self.v[x] = self.v[y];
        }

        let carry = self.v[x] & 0x0001 == 1;
        self.v[x] /= 2;

        if carry {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
    }
    fn op_8xy7(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);

        self.v[x] = result;
        self.v[0xF] = if borrow { 0 } else { 1 };
    }
    fn op_8xye(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        if self.quirks.shifting {
            self.v[x] = self.v[y];
        }

        let carry = self.v[x] & 0x80 != 0;
        self.v[x] = self.v[x].wrapping_mul(2);

        if carry {
            self.v[0xF] = 1;
        } else {
            self.v[0xF] = 0;
        }
    }
    fn op_9xy0(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 2;
        }
    }
    fn op_annn(&mut self, opcode : u16) {
        self.i = opcode & 0x0FFF;
    }
    fn op_bnnn(&mut self, opcode : u16) {
        let nnn  = opcode & 0x0FFF;

        self.pc = nnn + self.v[0] as u16;
    }
    fn op_cxkk(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let kk = (opcode & 0x00FF) as u8;

        let rnd_value = rand::rng().random_range(0..=255) as u8;
        self.v[x as usize] = rnd_value & kk;
    }
    fn op_dxyn(&mut self, opcode : u16) {
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        self.v[0xF] = 0;

        let mut pos_x = self.v[x] % 64;
        let mut pos_y = self.v[y] % 32;

        for i in 0..n {
            let index = self.i + i as u16;
            let byte = self.mem.read_byte(index).reverse_bits();

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
    fn op_ex9e(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        if self.keypad[self.v[x] as usize].curr_state {
            self.pc += 2;
        }
    }
    fn op_exa1(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        if !self.keypad[self.v[x] as usize].curr_state {
            self.pc += 2;
        }
    }
    fn op_fx07(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;

        self.v[x as usize] = self.delay_timer;
    }
    fn op_fx0a(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        for i in 0..16 {
            if self.keypad[i].prev_state && !self.keypad[i].curr_state {
                self.v[x] = i as u8;
                return;
            }
        }

        // Dont move to next instruction
        self.pc -= 2;
    }
    fn op_fx15(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;

        self.delay_timer = self.v[x as usize];
    }
    fn op_fx18(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;

        self.sound_timer = self.v[x as usize];
    }
    fn op_fx1e(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;

        self.i += self.v[x as usize] as u16;
    }
    fn op_fx29(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;

        self.i = FONTSET_START + self.v[x as usize] as u16;
    }
    fn op_fx33(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as u8;

        let value = self.v[x as usize];
        self.mem.write_byte(self.i, value / 100);
        self.mem.write_byte(self.i + 1, (value / 10) % 10);
        self.mem.write_byte(self.i + 2, value % 10);
    }
    fn op_fx55(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        for i in 0..=x {
            self.mem.write_byte(self.i + i as u16, self.v[i]);
        }

        if self.quirks.memory_increment {
            self.i = self.i + x as u16 + 1;
        }
    }
    fn op_fx65(&mut self, opcode : u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;

        for i in 0..=x {
            self.v[i] = self.mem.read_byte(self.i + i as u16);
        }

        if self.quirks.memory_increment {
            self.i = self.i + x as u16 + 1;
        }
    }
}