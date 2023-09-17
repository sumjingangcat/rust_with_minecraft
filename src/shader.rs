// Vertex Shader
// => 3D Space -> 2D Space (Position of Vertex)

// Fragment Shader
// => 2D Space (Position) -> 2D Space (Color of Pixel)
use gl;

use std::{ffi::{CString, CStr}, cell::RefCell, collections::HashMap};

use crate::gl_call;

#[derive(Debug)]
pub struct ShaderPart{
    id: u32,
}

impl ShaderPart{
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<ShaderPart, String> {
        let id: u32 = shader_from_source(source, kind)?;
        Ok(ShaderPart{id})
    }

    pub fn from_vert_source(source: &CStr) -> Result<ShaderPart, String> {
        ShaderPart::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<ShaderPart, String> {
        ShaderPart::from_source(source, gl::FRAGMENT_SHADER)
    }
}

// When we drop ShaderPart, we want to delete the shader
impl Drop for ShaderPart{
    fn drop(&mut self){
        gl_call!(gl::DeleteShader(self.id));
    }
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
    let id : u32 = gl_call!(gl::CreateShader(kind));
    gl_call!(gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null()));
    gl_call!(gl::CompileShader(id));

    let mut success: gl::types::GLint = 1;
    gl_call!(gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success));

    if success == 0{
        let mut len: gl::types::GLint = 0;
        gl_call!(gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len));

        let error = create_whitespace_cstring_with_len(len as usize);

        gl_call!(gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar,
        ));

        return Err(error.to_string_lossy().into_owned());
    }

    Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString{
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));

    unsafe {CString::from_vec_unchecked(buffer)}
}

#[derive(Debug)]

pub struct ShaderProgram{
    id: u32,
    uniform_cache: RefCell<HashMap<String, i32>>,
}

impl ShaderProgram{
    pub fn use_program(&self){
        gl_call!(gl::UseProgram(self.id));
    }

    pub fn from_shaders(vertex: ShaderPart, fragment: ShaderPart) -> Result<ShaderProgram, String>{
        let program_id = gl_call!(gl::CreateProgram());

        gl_call!(gl::AttachShader(program_id, vertex.id));
        gl_call!(gl::AttachShader(program_id, fragment.id));
        gl_call!(gl::LinkProgram(program_id));

        let mut success: gl::types::GLint = 1;
        gl_call!(gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success));

        if success == 0{
            let mut len: gl::types::GLint = 0;
            gl_call!(gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len));

            let error: CString = create_whitespace_cstring_with_len(len as usize);

            gl_call!(gl::GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            ));

            return Err(error.to_string_lossy().into_owned());
        }
        
        gl_call!(gl::DetachShader(program_id, vertex.id));
        gl_call!(gl::DetachShader(program_id, fragment.id));

        Ok(ShaderProgram {id : program_id, uniform_cache: RefCell::new(HashMap::new())})
    }
}

impl Drop for ShaderProgram{
    fn drop(&mut self){
        gl_call!(gl::DeleteProgram(self.id));
    }
}