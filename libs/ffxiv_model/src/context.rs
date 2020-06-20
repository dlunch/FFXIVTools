use alloc::sync::Arc;

use crate::shader_holder::ShaderHolder;
use crate::texture_cache::TextureCache;

use renderer::{Renderer, Texture, TextureFormat};

pub struct Context {
    pub(crate) shader_holder: ShaderHolder,
    pub(crate) texture_cache: TextureCache,
    pub(crate) empty_texture: Arc<Texture>,
}

impl Context {
    pub async fn new(renderer: &Renderer) -> Self {
        Self {
            shader_holder: ShaderHolder::new(renderer),
            texture_cache: TextureCache::new(),
            empty_texture: Self::create_empty_texture(renderer).await,
        }
    }

    async fn create_empty_texture(renderer: &Renderer) -> Arc<Texture> {
        Arc::new(Texture::with_texels(renderer, 1, 1, &[0, 0, 0, 0], TextureFormat::Rgba8Unorm).await)
    }
}
