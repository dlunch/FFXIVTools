use ffxiv_parser::{BufferItemType, BufferItemUsage, MtrlParameterType, Tex, TextureType};
use renderer::{CompressedTextureFormat, Renderer, Texture, TextureFormat, VertexItemType};

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
        BufferItemUsage::Bitangent => "Bitangent",
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

fn convert_compressed_texture_format(texture_type: TextureType) -> CompressedTextureFormat {
    match texture_type {
        TextureType::DXT1 => CompressedTextureFormat::BC1,
        TextureType::DXT3 => CompressedTextureFormat::BC2,
        TextureType::DXT5 => CompressedTextureFormat::BC3,
        _ => panic!(),
    }
}

pub async fn load_texture(renderer: &Renderer, tex: &Tex) -> Texture {
    if tex.texture_type() == TextureType::BGRA {
        Texture::new(&renderer, tex.width() as u32, tex.height() as u32, tex.data(0), TextureFormat::Bgra8Unorm).await
    } else {
        Texture::new_compressed(
            &renderer,
            tex.width() as u32,
            tex.height() as u32,
            tex.data(0),
            convert_compressed_texture_format(tex.texture_type()),
        )
        .await
    }
}
