use ffxiv_parser::{BufferItemType, BufferItemUsage, MtrlParameterType};
use renderer::VertexItemType;

pub fn convert_buffer_type(item_type: BufferItemType) -> VertexItemType {
    match item_type {
        BufferItemType::UByte4 => VertexItemType::UByte4,
        BufferItemType::UByte4n => VertexItemType::UByte4,
        BufferItemType::Float2 => VertexItemType::Float2,
        BufferItemType::Float3 => VertexItemType::Float3,
        BufferItemType::Float4 => VertexItemType::Float4,
        BufferItemType::Half2 => VertexItemType::Half2,
        BufferItemType::Half4 => VertexItemType::Half4,
        _ => panic!(),
    }
}

pub fn convert_buffer_usage(usage: BufferItemUsage) -> &'static str {
    match usage {
        BufferItemUsage::Position => "Position",
        BufferItemUsage::BoneWeight => "BoneWeight",
        BufferItemUsage::BoneIndex => "BoneIndex",
        BufferItemUsage::Normal => "Normal",
        BufferItemUsage::TexCoord => "TexCoord",
        BufferItemUsage::Tangent => "Tangent",
        BufferItemUsage::BiTangent => "BiTangent",
        BufferItemUsage::Color => "Color",
    }
}

pub fn convert_texture_name(parameter_type: MtrlParameterType) -> &'static str {
    match parameter_type {
        MtrlParameterType::Normal => "Normal",
        MtrlParameterType::Mask => "Mask",
        MtrlParameterType::Diffuse => "Diffuse",
        MtrlParameterType::Specular => "Specular",
        MtrlParameterType::Catchlight => "Catchlight",
    }
}
