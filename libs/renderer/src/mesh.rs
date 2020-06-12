use alloc::vec::Vec;

use crate::{buffer::Buffer, Renderer, VertexFormat};

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
    pub(crate) vertex_buffers: Vec<Buffer>,
    pub(crate) strides: Vec<usize>,
    pub(crate) index_buffer: Buffer,
    pub(crate) vertex_formats: Vec<VertexFormat>,
}

impl Mesh {
    pub async fn new(renderer: &Renderer, vertex_data: &[&[u8]], strides: &[usize], index_data: &[u8], vertex_formats: Vec<VertexFormat>) -> Self {
        let mut vertex_buffers = Vec::with_capacity(vertex_data.len());
        for vertex_datum in vertex_data {
            let buffer = renderer.buffer_pool.alloc(&renderer.device, vertex_datum.len());
            buffer.write(&renderer.device, vertex_datum).await.unwrap();

            vertex_buffers.push(buffer);
        }
        let index_buffer = renderer.buffer_pool.alloc(&renderer.device, index_data.len());
        index_buffer.write(&renderer.device, index_data).await.unwrap();

        Self {
            vertex_buffers,
            strides: Vec::from(strides),
            index_buffer,
            vertex_formats,
        }
    }
}
