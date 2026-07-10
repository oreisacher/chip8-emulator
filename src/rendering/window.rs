use glfw::{Action, Context, Glfw, GlfwReceiver, Key, PWindow, WindowEvent};
use glfw::WindowMode::Windowed;

pub struct Window {
    width : u32,
    height : u32,

    pub glfw_window : PWindow,
    pub events : GlfwReceiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn new(glfw : &mut Glfw, width : u32, height : u32, title : String) -> Window {
        // Create window
        let (mut window, events) = glfw.create_window(width, height, &title, Windowed).expect("Window creation failed.");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // Load OpenGL
        gl::load_with(|s| window.get_proc_address(s).expect("OpenGL init failed.") as *const _);

        Window {
            width,
            height,
            glfw_window : window,
            events
        }
    }

    pub fn swap_buffers(&mut self) {
        self.glfw_window.swap_buffers();
    }

    pub fn set_title(&mut self, title : String) {
        self.glfw_window.set_title(&title);
    }
}
