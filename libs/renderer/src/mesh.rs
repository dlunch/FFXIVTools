pub struct Mesh {
    vertex: wgpu::Buffer,
    index: wgpu::Buffer,
    index_count: usize,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertex: &[u8], index: &[u8], index_count: usize) -> Self {
        let vertex = device.create_buffer_with_data(vertex, wgpu::BufferUsage::VERTEX);
        let index = device.create_buffer_with_data(index, wgpu::BufferUsage::INDEX);

        Self { vertex, index, index_count }
    }

    pub fn vertex(&self) -> &wgpu::Buffer {
        &self.vertex
    }

    pub fn index(&self) -> &wgpu::Buffer {
        &self.index
    }

    pub fn index_count(&self) -> usize {
        self.index_count
    }
}
