use std::ffi::c_void;

use crate::gl_call;

#[derive(Clone)]

pub struct QuadProps{
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub color: (f32, f32, f32, f32),
}

pub struct Renderer{
    vertices: Vec<f32>,
    vbo: u32, // Vertex Buffer Object :: GPU memory buffer
    // if we draw a lot of objects, we can use one VBO
    vao: u32, // Vertex Array Object :: contains information about VBO, composed of VBOs
    // contains information about VBO
}

impl Renderer {
    pub fn new(capacity: usize)-> Self {
        let mut vertices = Vec::new();
        vertices.reserve(capacity);

        // Setup VBO
        let mut vbo = 0;
        gl_call!(gl::CreateBuffers(1, &mut vbo));

        gl_call!(gl::NamedBufferData(
            vbo,
            (capacity * std::mem::size_of::<f32>()) as isize, // size of buffer
            // why isize? , Find it! 
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        ));

        // Setup VAO
        let mut vao = 0;
        let mut binding_index_pos = 0;
        let mut binding_index_color = 1;

        // (x, y, r, g, b, a) * 6

        // Position
        gl_call!(gl::CreateVertexArrays(1, &mut vao));
        
        gl_call!(gl::EnableVertexArrayAttrib(vao, 0));
        gl_call!(gl::VertexArrayAttribFormat(
            vao,
            0, // index
            2, // size
            gl::FLOAT,
            gl::FALSE,
            0, // offset
        ));

        gl_call!(gl::VertexArrayAttribBinding(vao, 0, binding_index_pos));
        gl_call!(gl::VertexArrayVertexBuffer( // binding buffer to VAO
            vao,
            binding_index_pos,
            vbo,
            0,
            (6 * std::mem::size_of::<f32>()) as i32, // stride
        ));

        // Color
        gl_call!(gl::EnableVertexArrayAttrib(vao, 1));
        gl_call!(gl::VertexArrayAttribFormat(
            vao,
            1, // index
            4, // size
            gl::FLOAT,
            gl::FALSE,
            (2 * std::mem::size_of::<f32>()) as u32, // offset
        ));
        
        gl_call!(gl::VertexArrayAttribBinding(vao, 1, binding_index_color));
        gl_call!(gl::VertexArrayVertexBuffer(
            vao,
            binding_index_color,
            vbo,
            0,
            (6 * std::mem::size_of::<f32>()) as i32, // stride
        ));

        return Renderer {
            vertices,
            vbo,
            vao,
        };
    }

    // 데이터에 넣는 것을 "배치"라고 함. -> into english :  "batch"
    pub fn begin_batch(&mut self){
        self.vertices.clear();
    }

    pub fn submit_quad(&mut self, quad_props: QuadProps){
        let QuadProps{position: (x, y), size:(w, h), color: (r, g, b, a)} = quad_props;

        self.vertices.extend_from_slice(&[x, y, r, g, b, a]);
        self.vertices.extend_from_slice(&[x + w, y, r, g, b, a]);
        self.vertices.extend_from_slice(&[x + w, y + h, r, g, b, a]);
        self.vertices.extend_from_slice(&[x, y, r, g, b, a]);
        self.vertices.extend_from_slice(&[x + w, y + h, r, g, b, a]);
        self.vertices.extend_from_slice(&[x, y + h, r, g, b, a]);
    }

    pub fn end_batch(&mut self){
        gl_call!(gl::NamedBufferSubData(
            self.vbo,
            0,
            (self.vertices.len() * std::mem::size_of::<f32>()) as isize,
            self.vertices.as_ptr() as *mut c_void,
        ));

        gl_call!(gl::BindVertexArray(self.vao)); // vao binding : must be called before draw call
        gl_call!(gl::DrawArrays(
            gl::TRIANGLES,
            0,
            self.vertices.len() as i32,
        ));
    }

}

// 1. glBindAttribLocation
// 2. C/C++ : glVertexAttribPointer
// 3. glEnableVertexAttribArray
// 4. glDrawArrays
// 5. glDisableVertexAttribArray