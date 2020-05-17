use crate::shader_holder::ShaderHolder;
use crate::texture_cache::TextureCache;

use renderer::Renderer;

pub struct Context {
    pub(crate) shader_holder: ShaderHolder,
    pub(crate) texture_cache: TextureCache,
}

impl Context {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            shader_holder: ShaderHolder::new(renderer),
            texture_cache: TextureCache::new(),
        }
    }
}
