use alloc::{string::String, sync::Arc, vec, vec::Vec};

use futures::channel::oneshot;
use hashbrown::HashMap;
use spinning_top::Spinlock;

use ffxiv_parser::{Tex, TextureType};
use renderer::{CompressedTextureFormat, Renderer, Texture, TextureFormat};
use sqpack::{Package, Result};

pub struct TextureCache {
    waiters: Spinlock<HashMap<String, Vec<oneshot::Sender<bool>>>>,
    textures: Spinlock<HashMap<String, Arc<Texture>>>,
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            waiters: Spinlock::new(HashMap::new()),
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
        let should_fetch;
        let (tx, rx) = oneshot::channel();
        {
            let mut waiters = self.waiters.lock();
            if waiters.contains_key(&texture_path) {
                waiters.get_mut(&texture_path).unwrap().push(tx);
                should_fetch = false;
            } else {
                waiters.insert(texture_path.clone(), vec![tx]);
                should_fetch = true;
            }
        }
        if should_fetch {
            let tex = Tex::new(package, &texture_path).await?;
            let texture = Arc::new(Self::load_texture(renderer, &tex).await);

            {
                let mut textures = self.textures.lock();
                textures.insert(texture_path.clone(), texture);
            }

            let waiters = self.waiters.lock().remove(&texture_path).unwrap();
            for waiter in waiters {
                waiter.send(true).unwrap();
            }
        }

        let _ = rx.await;

        Ok(self.textures.lock().get(&texture_path).unwrap().clone())
    }

    async fn load_texture(renderer: &Renderer, tex: &Tex) -> Texture {
        if tex.texture_type() == TextureType::BGRA {
            Texture::with_texels(&renderer, tex.width() as u32, tex.height() as u32, tex.data(0), TextureFormat::Bgra8Unorm).await
        } else {
            Texture::with_compressed_texels(
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
