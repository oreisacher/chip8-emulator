use std::ffi::CString;
use gl::types::{GLchar, GLint, GLuint};

#[derive(Debug)]
pub struct Shader {
    pub id : GLuint,
}

impl Shader {
    pub fn new(vert_src : &str, frag_src : &str) -> Shader {
        let shader : GLuint;

        unsafe {
            let mut success = gl::FALSE as GLint;
            let mut info_log = Vec::with_capacity(512);
            info_log.set_len(512 - 1);

            // Compile Vertex Shader
            let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let raw_c_vert_str = CString::new(vert_src.as_bytes()).unwrap();
            gl::ShaderSource(vert_shader, 1, &raw_c_vert_str.as_ptr(), std::ptr::null());
            gl::CompileShader(vert_shader);

            // Check for compilation errors
            gl::GetShaderiv(vert_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(vert_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }

            // Compile Fragment Shader
            let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let raw_c_frag_str = CString::new(frag_src.as_bytes()).unwrap();
            gl::ShaderSource(frag_shader, 1, &raw_c_frag_str.as_ptr(), std::ptr::null());
            gl::CompileShader(frag_shader);

            // Check for compilation errors
            gl::GetShaderiv(frag_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(frag_shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }

            // Link Shader
            shader = gl::CreateProgram();
            gl::AttachShader(shader, vert_shader);
            gl::AttachShader(shader, frag_shader);
            gl::LinkProgram(shader);

            // Check for linking errors
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(shader, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER::PROGRAM::LINKING_FAILED\n{}", str::from_utf8(&info_log).unwrap());
            }

            gl::DeleteShader(vert_shader);
            gl::DeleteShader(frag_shader);
        }

        Shader { id : shader }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_int(&self, name : &str, value : GLint) {
        unsafe {
            let cname = CString::new(name).expect("Unable to create CString.");
            gl::Uniform1i(gl::GetUniformLocation(self.id, cname.as_ptr()), value);
        }
    }

    pub fn set_vec3(&self, name : &str, value : [f32; 3]) {
        unsafe {
            let cname = CString::new(name).expect("Unable to create CString.");
            gl::Uniform3f(gl::GetUniformLocation(self.id, cname.as_ptr()), value[0], value[1], value[2]);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}