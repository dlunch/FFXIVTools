use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use ffxiv_parser::{Tex, TextureType};
use renderer::{CompressedTextureFormat, Renderer, Texture, TextureFormat};
use sqpack_reader::{Package, Result};

pub struct TextureCache {
    textures: RwLock<HashMap<String, Arc<Texture>>>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            textures: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get_or_read(&self, renderer: &Renderer, package: &dyn Package, texture_path: String) -> Result<Arc<Texture>> {
        {
            let textures = self.textures.read().await;
            if let Some(x) = textures.get(&texture_path) {
                return Ok(x.clone());
            }
        }
        // TODO wait for fetched but incomplete same request.

        let tex = Tex::new(package, &texture_path).await?;
        let texture = Arc::new(Self::load_texture(renderer, &tex).await);

        let mut textures = self.textures.write().await;
        textures.insert(texture_path, texture.clone());

        Ok(texture)
    }

    async fn load_texture(renderer: &Renderer, tex: &Tex) -> Texture {
        if tex.texture_type() == TextureType::BGRA {
            Texture::new(&renderer, tex.width() as u32, tex.height() as u32, tex.data(0), TextureFormat::Bgra8Unorm).await
        } else {
            Texture::new_compressed(
                &renderer,
                tex.width() as u32,
                tex.height() as u32,
                tex.data(0),
                Self::convert_compressed_texture_format(tex.texture_type()),
            )
            .await
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
}
