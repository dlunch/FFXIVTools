use super::VertexFormat;

pub struct Mesh {
    pub(crate) vertex_buffers: Vec<wgpu::Buffer>,
    strides: Vec<usize>,
    pub(crate) index: wgpu::Buffer,
    pub(crate) index_count: usize,

    attributes: Vec<Vec<wgpu::VertexAttributeDescriptor>>,
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

        let attributes = vertex_formats.into_iter().map(|x| x.into_wgpu_attributes()).collect::<Vec<_>>();

        Self {
            vertex_buffers,
            strides: Vec::from(strides),
            index,
            index_count,
            attributes,
        }
    }

    pub fn vertex_descriptors(&self) -> Vec<wgpu::VertexBufferDescriptor> {
        self.strides
            .iter()
            .enumerate()
            .map(|(i, x)| wgpu::VertexBufferDescriptor {
                stride: *x as wgpu::BufferAddress,
                step_mode: wgpu::InputStepMode::Vertex,
                attributes: self.attributes[i].as_ref(),
            })
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn index_format(&self) -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint16
    }
}
