use gl::types::{GLint, GLuint};
use crate::framebuffer::Framebuffer;
use crate::shader::Shader;

#[derive(Debug)]
pub struct Renderer {
    shader : Shader,
    texture : GLuint,
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

    in vec2 uv;

    out vec4 FragColor;

    void main() {
        float cell = texture(tex, uv).r;

        if (cell > 0.0f) {
            FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        } else {
            FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
    }
"#;

impl Renderer {
    pub fn new() -> Renderer {
        // Init Shader
        let shader = Shader::new(VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        }

        let mut texture = 0;
        unsafe {
            // Init texture
            gl::GenTextures(1, &mut texture);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::R8 as GLint,
                           64,
                           32,
                           0,
                           gl::RED,
                           gl::UNSIGNED_BYTE,
                           std::ptr::null());

            gl::Uniform1i(gl::GetUniformLocation(shader.id, b"tex\0".as_ptr() as *const _), 0);
        }

        let mut vao = 0;
        unsafe {
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
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0, 0, 0, 64, 32, gl::RED, gl::UNSIGNED_BYTE,
                fb.pixels.as_ptr() as *const _
            );

            gl::UseProgram(self.shader.id);

            // Draw to screen
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
            gl::DeleteTextures(1, &self.texture);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}