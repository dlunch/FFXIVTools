use crate::shader_holder::ShaderHolder;

use renderer::Renderer;

pub struct Context {
    pub(crate) shader_holder: ShaderHolder,
}

impl Context {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            shader_holder: ShaderHolder::new(renderer),
        }
    }
}
