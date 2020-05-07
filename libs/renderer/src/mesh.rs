use crate::VertexFormat;

pub struct Mesh {
    pub(crate) vertex_buffers: Vec<wgpu::Buffer>,
    pub(crate) strides: Vec<usize>,
    pub(crate) index: wgpu::Buffer,
    pub(crate) index_count: usize,
    pub(crate) vertex_formats: Vec<VertexFormat>,
}

impl Mesh {
    pub fn new(
        device: &wgpu::Device,
        vertex_buffers: &[&[u8]],
        strides: &[usize],
        index: &[u8],
        index_count: usize,
        vertex_formats: Vec<VertexFormat>,
    ) -> Self {
        let vertex_buffers = vertex_buffers
            .iter()
            .map(|x| device.create_buffer_with_data(x, wgpu::BufferUsage::VERTEX))
            .collect::<Vec<_>>();
        let index = device.create_buffer_with_data(index, wgpu::BufferUsage::INDEX);

        Self {
            vertex_buffers,
            strides: Vec::from(strides),
            index,
            index_count,
            vertex_formats,
        }
    }
}
