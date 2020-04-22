use super::VertexFormat;

pub struct Mesh {
    pub(crate) vertex: wgpu::Buffer,
    stride: usize,
    pub(crate) index: wgpu::Buffer,
    pub(crate) index_count: usize,

    attributes: Vec<wgpu::VertexAttributeDescriptor>,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertex: &[u8], stride: usize, index: &[u8], index_count: usize, vertex_format: VertexFormat) -> Self {
        let vertex = device.create_buffer_with_data(vertex, wgpu::BufferUsage::VERTEX);
        let index = device.create_buffer_with_data(index, wgpu::BufferUsage::INDEX);

        let attributes = vertex_format.into_wgpu_attributes();

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
