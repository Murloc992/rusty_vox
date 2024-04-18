use gl;
use std::fs;
use std::env;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn load(&mut self, vert_path: &str, frag_path: &str) -> u32 {
        println!("{}", env::current_dir().unwrap().join(vert_path).to_string_lossy());
        let vert_content = fs::read_to_string(env::current_dir().unwrap().join(vert_path)).expect("Vert file not found!");
        let frag_content = fs::read_to_string(env::current_dir().unwrap().join(frag_path)).expect("Frag file not found!");

        let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };
        let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

        let shader_id = Self::compile_and_link(vertex_shader, &vert_content, fragment_shader, &frag_content);
        self.id = shader_id;
        return shader_id;
    }

    fn compile_and_link(
        vert_shader_id: u32,
        vert_shader_str: &str,
        frag_shader_id: u32,
        frag_shader_str: &str,
    ) -> u32 {
        let shader_program = unsafe { gl::CreateProgram() };
        Self::compile(vert_shader_id, &vert_shader_str);
        Self::compile(frag_shader_id, &frag_shader_str);
        unsafe {
            gl::AttachShader(shader_program, vert_shader_id);
            gl::AttachShader(shader_program, frag_shader_id);
            gl::LinkProgram(shader_program);

            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }

            gl::DetachShader(shader_program, vert_shader_id);
            gl::DetachShader(shader_program, frag_shader_id);
            gl::DeleteShader(vert_shader_id);
            gl::DeleteShader(frag_shader_id);
        }

        return shader_program;
    }

    fn compile(shader_id: u32, shader_str: &str) {
        let strlen: i32 = shader_str.len() as i32;
        unsafe {
            gl::ShaderSource(
                shader_id,
                1,
                shader_str.as_bytes().as_ptr().cast(),
                &strlen,
            );
            gl::CompileShader(shader_id);
            let mut success = 0;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut log_len = 0_i32;
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                gl::GetShaderInfoLog(shader_id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Shader Compile Error: {}", String::from_utf8_lossy(&v));
            }
        }
    }

    pub fn use_as_current(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}
