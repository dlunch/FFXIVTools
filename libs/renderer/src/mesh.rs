pub struct Mesh {
    pub(crate) vertex: wgpu::Buffer,
    stride: usize,
    pub(crate) index: wgpu::Buffer,
    pub(crate) index_count: usize,

    attributes: Vec<wgpu::VertexAttributeDescriptor>,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertex: &[u8], stride: usize, index: &[u8], index_count: usize) -> Self {
        let vertex = device.create_buffer_with_data(vertex, wgpu::BufferUsage::VERTEX);
        let index = device.create_buffer_with_data(index, wgpu::BufferUsage::INDEX);

        let attributes = vec![
            wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float4,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float2,
                offset: 4 * 4,
                shader_location: 1,
            },
        ];

        Self {
            vertex,
            stride,
            index,
            index_count,
            attributes,
        }
    }

    pub fn buffer_descriptor(&self) -> wgpu::VertexBufferDescriptor {
        wgpu::VertexBufferDescriptor {
            stride: self.stride as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: self.attributes.as_ref(),
        }
    }

    #[inline]
    pub fn index_format(&self) -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint16
    }
}
