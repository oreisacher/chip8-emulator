use std::time::{Duration, Instant};
use glfw::{Action, Glfw, Key};
use crate::chip8::Chip8;
use crate::renderer::Renderer;
use crate::sound::Sound;
use crate::window::Window;

pub struct Application {
    chip8 : Chip8,
    sound : Sound,
    renderer : Renderer,
    window : Window,
    glfw : Glfw,
}

const INSTRUCTION_PER_SECOND: i32 = 700;

impl Application {
    pub fn new(rom_path : String) -> Application {
        // Init GLFW
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).expect("GLFW init Failed");

        // Create window
        let window = Window::new(&mut glfw, 700, 350, "Chip8 Emulator".to_string());

        // Disable V-Sync
        glfw.set_swap_interval(glfw::SwapInterval::None);

        // Create Chip8 and load rom
        let mut chip8 = Chip8::new();
        chip8.load_rom(rom_path);

        Application {
            chip8,
            sound : Sound::new(),
            renderer : Renderer::new(),
            glfw,
            window
        }
    }

    pub fn run(&mut self) {
        let mut timer = Instant::now();
        let mut frame_time: Instant;
        let target_frame_time = Duration::from_secs_f64(1.0/(INSTRUCTION_PER_SECOND as f64));

        while !self.window.glfw_window.should_close() {
            frame_time = Instant::now();

            self.chip8.update_keyboard_states();
            self.glfw.poll_events();
            self.process_events();

            self.chip8.cycle();

            // Update Chip8 Timers
            if timer.elapsed() >= self.chip8.timer_interval {
                timer = Instant::now();
                self.chip8.update_timers();
            }

            // Update sound
            if self.chip8.playing_sound() {
                self.sound.play_sound();
            } else {
                self.sound.stop_sound();
            }

            self.renderer.draw(&self.chip8.fb);

            self.window.swap_buffers();

            let elapsed = frame_time.elapsed();
            if elapsed < target_frame_time {
                std::thread::sleep(target_frame_time - elapsed);
            }
        }
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.window.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    self.renderer.resize(width, height);
                },
                glfw::WindowEvent::Key(key, _, action, _) => {
                    let mut chip8_key;

                    match key {
                        Key::Num1 => chip8_key = 0x1,
                        Key::Num2 => chip8_key = 0x2,
                        Key::Num3 => chip8_key = 0x3,
                        Key::Num4 => chip8_key = 0xC,
                        Key::Q => chip8_key = 0x4,
                        Key::W => chip8_key = 0x5,
                        Key::E => chip8_key = 0x6,
                        Key::R => chip8_key = 0xD,
                        Key::A => chip8_key = 0x7,
                        Key::S => chip8_key = 0x8,
                        Key::D => chip8_key = 0x9,
                        Key::F => chip8_key = 0xE,
                        Key::Z => chip8_key = 0xA,
                        Key::X => chip8_key = 0x0,
                        Key::C => chip8_key = 0xB,
                        Key::V => chip8_key = 0xF,
                        _ => chip8_key = 0x0
                    }

                    if action == Action::Press || action == Action::Repeat {
                        self.chip8.press_key(chip8_key);
                    } else {
                        self.chip8.release_key(chip8_key);
                    }
                }
                _ => ()
            }
        }
    }
}