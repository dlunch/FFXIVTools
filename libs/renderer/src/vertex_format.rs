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
    item_type: VertexItemType,
    offset: usize,
}

impl VertexFormatItem {
    pub fn new(item_type: VertexItemType, offset: usize) -> Self {
        Self { item_type, offset }
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
            .enumerate()
            .map(|(i, x)| wgpu::VertexAttributeDescriptor {
                format: x.item_type.wgpu_type(),
                offset: x.offset as u64,
                shader_location: i as u32,
            })
            .collect::<Vec<_>>()
    }
}
