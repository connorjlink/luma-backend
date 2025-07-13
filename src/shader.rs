// (c) 2025 Connor J. Link. All Rights Reserved.
// Luma - shader.rs

use std::fs;
use std::ffi::{CString, CStr};
use std::ptr;

pub struct Shader
{
    shader_id: u32,
    shader_type: u32,
    program_id: u32,
}

impl Shader
{
    pub fn new(shader_type: u32, program_id: u32, filepath: &str) -> Result<Self, String>
    {
        let code = fs::read_to_string(filepath)
            .map_err(|_| format!("Error reading shader source file: {}", filepath))?;

        let shader_id = unsafe { gl::CreateShader(shader_type) };
        let c_code = CString::new(code).unwrap();

        unsafe
        {
            gl::ShaderSource(shader_id, 1, &c_code.as_ptr(), ptr::null());
            gl::CompileShader(shader_id);

            let mut status = 0;
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);
            if status == 0
            {
                let mut len = 0;
                gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0u8; len as usize];
                gl::GetShaderInfoLog(shader_id, len, ptr::null_mut(), buffer.as_mut_ptr() as *mut _);
                let log = String::from_utf8_lossy(&buffer);
                return Err(format!("Error compiling shader: {}", log));
            }

            gl::AttachShader(program_id, shader_id);
        }

        return Ok(Self
        {
            shader_id,
            shader_type,
            program_id,
        });
    }

    pub fn release(&self)
    {
        unsafe
        {
            gl::DetachShader(self.program_id, self.shader_id);
            gl::DeleteShader(self.shader_id);
        }
    }
}

pub struct ShaderFactory {
    program_id: u32,
    shaders: Vec<Shader>,
}

impl ShaderFactory {
    pub fn new(program_id: u32) -> Self {
        Self {
            program_id,
            shaders: Vec::new(),
        }
    }

    pub fn compile_shader(&mut self, shader_type: u32, filepath: &str) -> Result<(), String> {
        let shader = Shader::new(shader_type, self.program_id, filepath)?;
        self.shaders.push(shader);
        Ok(())
    }

    pub fn link(&mut self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.program_id);
        }
        for shader in &self.shaders {
            shader.release();
        }
        Ok(())
    }
}

pub struct ShaderProgram
{
    program_id: u32,
}

impl ShaderProgram {
    pub fn new(partial_filepath: &str) -> Result<Self, String>
    {
        let program_id = unsafe { gl::CreateProgram() };
        let mut factory = ShaderFactory::new(program_id);

        factory.compile_shader(gl::VERTEX_SHADER, &format!("{}.vertex.glsl", partial_filepath))?;
        factory.compile_shader(gl::FRAGMENT_SHADER, &format!("{}.fragment.glsl", partial_filepath))?;
        factory.link()?;

        return Ok(Self { program_id });
    }

    pub fn use_program(&self)
    {
        unsafe
        {
            gl::UseProgram(self.program_id);
        }
    }

    pub fn locate_uniform(&self, identifier: &str) -> i32
    {
        let c_str = CString::new(identifier).unwrap();
        return unsafe { gl::GetUniformLocation(self.program_id, c_str.as_ptr()) };
    }

    pub fn upload_matrix4fv(&self, matrix: &[f32; 16], identifier: &str)
    {
        let location = self.locate_uniform(identifier);
        unsafe 
        {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, matrix.as_ptr());
        }
    }
}

impl Drop for ShaderProgram
{
    fn drop(&mut self)
    {
        unsafe
        {
            gl::DeleteProgram(self.program_id);
        }
    }
}