use alloc::{string::String, sync::Arc};

use hashbrown::HashMap;
use spinning_top::Spinlock;

use ffxiv_parser::{Tex, TextureType};
use renderer::{CompressedTextureFormat, Renderer, Texture, TextureFormat};
use sqpack_reader::{Package, Result};

pub struct TextureCache {
    textures: Spinlock<HashMap<String, Arc<Texture>>>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            textures: Spinlock::new(HashMap::new()),
        }
    }

    pub async fn get_or_read(&self, renderer: &Renderer, package: &dyn Package, texture_path: String) -> Result<Arc<Texture>> {
        {
            let textures = self.textures.lock();
            if let Some(x) = textures.get(&texture_path) {
                return Ok(x.clone());
            }
        }
        // TODO wait for fetched but incomplete same request.

        let tex = Tex::new(package, &texture_path).await?;
        let texture = Arc::new(Self::load_texture(renderer, &tex).await);

        let mut textures = self.textures.lock();
        textures.insert(texture_path, texture.clone());

        Ok(texture)
    }

    async fn load_texture(renderer: &Renderer, tex: &Tex) -> Texture {
        if tex.texture_type() == TextureType::BGRA {
            Texture::new(
                &renderer,
                tex.width() as u32,
                tex.height() as u32,
                Some(tex.data(0)),
                TextureFormat::Bgra8Unorm,
            )
        } else {
            Texture::new_compressed(
                &renderer,
                tex.width() as u32,
                tex.height() as u32,
                tex.data(0),
                Self::convert_compressed_texture_format(tex.texture_type()),
            )
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
