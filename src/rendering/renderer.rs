use gl::types::{GLuint};
use crate::config::Config;
use crate::emulator::framebuffer::Framebuffer;
use super::shader::Shader;
use super::texture::Texture;

#[derive(Debug)]
pub struct Renderer {
    shader : Shader,
    texture : Texture,
    vao : GLuint
}

const VERTEX_SHADER_SOURCE : &str = r#"
    #version 430 core

    out vec2 uv;

    void main() {
        vec2 pos = vec2(
            (gl_VertexID & 1) * 2.0 - 1.0,
            (gl_VertexID >> 1) * 2.0 - 1.0
        );

        uv = vec2(pos.x * 0.5 + 0.5, 1.0 - (pos.y * 0.5 + 0.5));
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE : &str = r#"
    #version 430

    uniform sampler2D tex;
    uniform vec3 onColor;
    uniform vec3 offColor;

    in vec2 uv;

    out vec4 FragColor;

    void main() {
        float cell = texture(tex, uv).r;

        if (cell > 0.0f) {
            FragColor = vec4(onColor, 1.0);
        } else {
            FragColor = vec4(offColor, 1.0);
        }
    }
"#;

impl Renderer {
    pub fn new(config : &Config) -> Renderer {
        // --- Shader ---
        let shader = Shader::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);
        shader.bind();
        shader.set_int("tex", 0);
        shader.set_vec3("onColor", config.display.on_color);
        shader.set_vec3("offColor", config.display.off_color);

        // --- Texture ---
        let texture = Texture::new(64, 32, gl::R8, gl::RED);
        texture.bind();

        // --- OpenGL ---
        let mut vao = 0;
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::GenVertexArrays(1, &mut vao);
        }

        Renderer {
            shader,
            texture,
            vao
        }
    }

    pub fn draw(&self, fb : &Framebuffer) {
        unsafe {
            // Upload framebuffer data to texture
            self.texture.set_framebuffer_data(fb);

            // Draw texture to screen
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}