mod chip8;
mod memory;
mod framebuffer;
mod renderer;
mod window;
mod shader;
mod application;
mod sound;
mod texture;
mod keypad;

use crate::application::Application;

fn main() {
    let args : Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [rom_path]", args[0]);
        return;
    }

    let mut app = Application::new(args[1].clone());
    app.run();
}
