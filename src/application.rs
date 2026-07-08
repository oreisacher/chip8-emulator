use std::time::{Duration, Instant};
use glfw::{Action, Glfw, Key};
use crate::chip8::Chip8;
use crate::renderer::Renderer;
use crate::sound::Sound;
use crate::window::Window;

const CPU_CLOCK_HZ: i32 = 1000;
const TIMER_HZ: f64 = 60.0;

pub struct Application {
    chip8 : Chip8,
    sound : Sound,
    renderer : Renderer,
    window : Window,
    glfw : Glfw,
}

impl Application {
    pub fn new(rom_path : String) -> Application {
        // Init GLFW
        use glfw::fail_on_errors;
        let mut glfw = glfw::init(fail_on_errors!()).expect("GLFW init Failed");

        // Create window
        let mut window = Window::new(&mut glfw, 700, 350, "Chip8 Emulator (No rom)".to_string());

        // Disable V-Sync
        glfw.set_swap_interval(glfw::SwapInterval::None);

        // Create Chip8 and load rom
        let mut chip8 = Chip8::new();
        chip8.load_rom(rom_path);

        window.set_title(format!("Chip8 Emulator ({})", &chip8.loaded_rom));

        Application {
            chip8,
            sound : Sound::new(),
            renderer : Renderer::new(),
            glfw,
            window
        }
    }

    pub fn run(&mut self) {
        let cycles_per_tick = (CPU_CLOCK_HZ as f64 / TIMER_HZ).round() as u32;
        let target_tick_duration = Duration::from_secs_f64(1.0 / TIMER_HZ);

        while !self.window.glfw_window.should_close() {
            let tick_start = Instant::now();

            // --- Input ---
            self.glfw.poll_events();
            self.chip8.update_keyboard_states();
            self.process_events();

            // --- CPU ---
            for _ in 0..cycles_per_tick {
                self.chip8.cycle();
            }

            // --- Timers ---
            // Once per outer loop. Loop runs at fixed Hz (default 60)
            self.chip8.update_timers();

            // --- Sound ---
            if self.chip8.sound_timer > 0 {
                self.sound.play_sound();
            } else {
                self.sound.stop_sound();
            }

            // --- Render ---
            self.renderer.draw(&self.chip8.fb);
            self.window.swap_buffers();

            // Ensure that this loop runs at TIMER_HZ
            let elapsed = tick_start.elapsed();
            if elapsed < target_tick_duration {
                std::thread::sleep(target_tick_duration - elapsed);
            }
        }

        // Poll events once more to avoid a segfault
        self.glfw.poll_events();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.window.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    self.renderer.resize(width, height);
                },
                glfw::WindowEvent::Key(key, _, action, _) => {
                    let chip8_key : Option<u8> = match key {
                        Key::Num1 => Some(0x1),
                        Key::Num2 => Some(0x2),
                        Key::Num3 => Some(0x3),
                        Key::Num4 => Some(0xC),
                        Key::Q => Some(0x4),
                        Key::W => Some(0x5),
                        Key::E => Some(0x6),
                        Key::R => Some(0xD),
                        Key::A => Some(0x7),
                        Key::S => Some(0x8),
                        Key::D => Some(0x9),
                        Key::F => Some(0xE),
                        Key::Z => Some(0xA),
                        Key::X => Some(0x0),
                        Key::C => Some(0xB),
                        Key::V => Some(0xF),
                        _ => None
                    };

                    if let Some(key) = chip8_key {
                        if action == Action::Press || action == Action::Repeat {
                            self.chip8.press_key(key);
                        } else {
                            self.chip8.release_key(key);
                        }
                    }
                }
                _ => ()
            }
        }
    }
}
