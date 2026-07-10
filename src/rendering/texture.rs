use gl::types::{GLenum, GLint, GLsizei, GLuint};
use crate::emulator::framebuffer::Framebuffer;

#[derive(Debug)]
pub struct Texture {
    id : GLuint,
    width : GLsizei,
    height : GLsizei,
    internal_format : GLenum,
    format : GLenum
}

impl Texture {
    pub fn new(width : GLsizei, height : GLsizei, internal_format : GLenum, format : GLenum) -> Texture {
        // Init texture
        unsafe {
            let mut texture = 0;
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           internal_format as GLint,
                           width,
                           height,
                           0,
                           format,
                           gl::UNSIGNED_BYTE,
                           std::ptr::null());

            Texture {
                id : texture,
                width,
                height,
                internal_format,
                format
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn set_framebuffer_data(&self, framebuffer : &Framebuffer) {
        unsafe {
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0, 0, 0, self.width, self.height, self.format, gl::UNSIGNED_BYTE,
                framebuffer.pixels.as_ptr() as *const _
            );
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}