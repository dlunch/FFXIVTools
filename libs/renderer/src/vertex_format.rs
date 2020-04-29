pub enum VertexItemType {
    Float2,
    Float3,
    Float4,
    Half2,
    Half4,
}

impl VertexItemType {
    pub(crate) fn wgpu_type(&self) -> wgpu::VertexFormat {
        match self {
            VertexItemType::Float2 => wgpu::VertexFormat::Float2,
            VertexItemType::Float3 => wgpu::VertexFormat::Float3,
            VertexItemType::Float4 => wgpu::VertexFormat::Float4,
            VertexItemType::Half2 => wgpu::VertexFormat::Half2,
            VertexItemType::Half4 => wgpu::VertexFormat::Half4,
        }
    }
}

pub struct VertexFormatItem {
    shader_location: usize,
    item_type: VertexItemType,
    offset: usize,
}

impl VertexFormatItem {
    pub fn new(shader_location: usize, item_type: VertexItemType, offset: usize) -> Self {
        Self {
            shader_location,
            item_type,
            offset,
        }
    }
}

pub struct VertexFormat {
    items: Vec<VertexFormatItem>,
}

impl VertexFormat {
    pub fn new(items: Vec<VertexFormatItem>) -> Self {
        Self { items }
    }

    pub(crate) fn into_wgpu_attributes(self) -> Vec<wgpu::VertexAttributeDescriptor> {
        self.items
            .into_iter()
            .map(|x| wgpu::VertexAttributeDescriptor {
                format: x.item_type.wgpu_type(),
                offset: x.offset as u64,
                shader_location: x.shader_location as u32,
            })
            .collect::<Vec<_>>()
    }
}
