use gl;

pub struct BufferObject {
    buffer_id: u32,
}

pub struct Mesh {
    vao_id: u32,
    vbo_ids: Vec<u32>,
    ibo_id: u32,
    buffers: Vec<BufferObject>,
}

impl Mesh {
    pub fn create() {

    }

    pub fn set_buffer_data() {

    }
}