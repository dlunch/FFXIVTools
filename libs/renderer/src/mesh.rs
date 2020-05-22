use crate::{Renderer, VertexFormat};

pub struct MeshPart {
    pub(crate) begin: u32,
    pub(crate) count: u32,
}

impl MeshPart {
    pub fn new(begin: u32, count: u32) -> Self {
        Self { begin, count }
    }
}

pub struct Mesh {
    pub(crate) vertex_buffers: Vec<wgpu::Buffer>,
    pub(crate) strides: Vec<usize>,
    pub(crate) index: wgpu::Buffer,
    pub(crate) vertex_formats: Vec<VertexFormat>,
}

impl Mesh {
    pub fn new(renderer: &Renderer, vertex_buffers: &[&[u8]], strides: &[usize], index: &[u8], vertex_formats: Vec<VertexFormat>) -> Self {
        let vertex_buffers = vertex_buffers
            .iter()
            .map(|x| renderer.device.create_buffer_with_data(x, wgpu::BufferUsage::VERTEX))
            .collect::<Vec<_>>();
        let index = renderer.device.create_buffer_with_data(index, wgpu::BufferUsage::INDEX);

        Self {
            vertex_buffers,
            strides: Vec::from(strides),
            index,
            vertex_formats,
        }
    }
}
