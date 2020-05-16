use std::collections::HashMap;

use maplit::hashmap;

use renderer::{Renderer, Shader, ShaderBinding, ShaderBindingType};

pub struct ShaderHolder {
    vertex_shader: Shader,
    fragment_shaders: HashMap<&'static str, Shader>,
}

impl ShaderHolder {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            vertex_shader: Self::load_vertex_shader(renderer),
            fragment_shaders: hashmap! {
                "character.shpk" => Self::load_character_shader(renderer),
                "skin.shpk" => Self::load_skin_shader(renderer)
            },
        }
    }

    pub fn get_shaders<T: AsRef<str>>(&self, shader_name: T) -> (&Shader, &Shader) {
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
