pub struct Mesh {
    pub(crate) vertex: wgpu::Buffer,
    pub(crate) stride: usize,
    pub(crate) index: wgpu::Buffer,
    pub(crate) index_count: usize,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertex: &[u8], stride: usize, index: &[u8], index_count: usize) -> Self {
        let vertex = device.create_buffer_with_data(vertex, wgpu::BufferUsage::VERTEX);
        let index = device.create_buffer_with_data(index, wgpu::BufferUsage::INDEX);

        Self {
            vertex,
            stride,
            index,
            index_count,
        }
    }
}
