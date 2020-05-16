use std::collections::HashMap;
use std::sync::Arc;

use maplit::hashmap;

use renderer::{Renderer, Shader, ShaderBinding, ShaderBindingType};

pub struct ShaderHolder {
    vertex_shader: Arc<Shader>,
    fragment_shaders: HashMap<&'static str, Arc<Shader>>,
}

impl ShaderHolder {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            vertex_shader: Arc::new(Self::load_vertex_shader(renderer)),
            fragment_shaders: hashmap! {
                "character.shpk" => Arc::new(Self::load_character_shader(renderer)),
                "skin.shpk" => Arc::new(Self::load_skin_shader(renderer))
            },
        }
    }

    pub fn get_shaders<T: AsRef<str>>(&self, shader_name: T) -> (&Arc<Shader>, &Arc<Shader>) {
        (&self.vertex_shader, self.fragment_shaders.get(shader_name.as_ref()).unwrap())
    }

    fn load_vertex_shader(renderer: &Renderer) -> Shader {
        let vs_bytes = include_bytes!("../shaders/shader.vert.spv");

        Shader::new(
            &renderer,
            &vs_bytes[..],
            "main",
            hashmap! {"Locals" => ShaderBinding::new(0, ShaderBindingType::UniformBuffer)},
            hashmap! {
                "Position" => 0,
                "BoneWeight" => 1,
                "BoneIndex" => 2,
                "Normal" => 3,
                "TexCoord" => 4,
                "Bitangent" => 5,
                "Color" => 6,
            },
        )
    }

    fn load_character_shader(renderer: &Renderer) -> Shader {
        let fs_bytes = include_bytes!("../shaders/character.frag.spv");

        Shader::new(
            &renderer,
            &fs_bytes[..],
            "main",
            hashmap! {
                "Sampler" => ShaderBinding::new(1, ShaderBindingType::Sampler),
                "Normal" => ShaderBinding::new(2, ShaderBindingType::Texture2D),
                "ColorTable" => ShaderBinding::new(3, ShaderBindingType::Texture2D),
                "Mask" => ShaderBinding::new(4, ShaderBindingType::Texture2D),
            },
            HashMap::new(),
        )
    }

    fn load_skin_shader(renderer: &Renderer) -> Shader {
        let fs_bytes = include_bytes!("../shaders/skin.frag.spv");

        Shader::new(
            &renderer,
            &fs_bytes[..],
            "main",
            hashmap! {
                "Sampler" => ShaderBinding::new(1, ShaderBindingType::Sampler),
                "Normal" => ShaderBinding::new(2, ShaderBindingType::Texture2D),
                "Diffuse" => ShaderBinding::new(3, ShaderBindingType::Texture2D),
            },
            HashMap::new(),
        )
    }
}