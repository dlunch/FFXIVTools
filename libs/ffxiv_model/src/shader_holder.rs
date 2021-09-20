use alloc::{format, string::String, sync::Arc};
use core::iter::FromIterator;

use hashbrown::HashMap;

use renderer::{Renderer, Shader, ShaderBinding, ShaderBindingType, ShaderStage};

#[derive(Eq, PartialEq, Hash)]
pub enum ShaderType {
    Character,
    Iris,
    Hair,
    Skin,
}

pub struct ShaderHolder {
    shaders: HashMap<ShaderType, Arc<Shader>>,
}

impl ShaderHolder {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            shaders: HashMap::from_iter([
                (ShaderType::Character, Arc::new(Self::load_character_shader(renderer))),
                (ShaderType::Iris, Arc::new(Self::load_iris_shader(renderer))),
                (ShaderType::Hair, Arc::new(Self::load_hair_shader(renderer))),
                (ShaderType::Skin, Arc::new(Self::load_skin_shader(renderer))),
            ]),
        }
    }

    pub fn shader(&self, shader: ShaderType) -> Arc<Shader> {
        self.shaders.get(&shader).unwrap().clone()
    }

    fn compose_shader(fragment: &str) -> String {
        format!(
            "{}\n{}\n{}",
            include_str!("../shaders/vertex.wgsl"),
            include_str!("../shaders/frag_common.wgsl"),
            fragment
        )
    }

    fn load_character_shader(renderer: &Renderer) -> Shader {
        let shader = Self::compose_shader(include_str!("../shaders/character.wgsl"));

        Shader::new(
            renderer,
            &shader,
            "vs_main",
            "fs_main",
            &[
                ("Mvp", ShaderBinding::new(ShaderStage::Vertex, 0, ShaderBindingType::UniformBuffer)),
                (
                    "BoneTransformsUniform",
                    ShaderBinding::new(ShaderStage::Vertex, 1, ShaderBindingType::UniformBuffer),
                ),
                ("Sampler", ShaderBinding::new(ShaderStage::Fragment, 10, ShaderBindingType::Sampler)),
                ("Normal", ShaderBinding::new(ShaderStage::Fragment, 11, ShaderBindingType::Texture2D)),
                ("ColorTable", ShaderBinding::new(ShaderStage::Fragment, 12, ShaderBindingType::Texture2D)),
                ("Mask", ShaderBinding::new(ShaderStage::Fragment, 13, ShaderBindingType::Texture2D)),
            ],
            &[
                ("Position", 0),
                ("BoneWeight", 1),
                ("BoneIndex", 2),
                ("Normal", 3),
                ("TexCoord", 4),
                ("BiTangent", 5),
                ("Color", 6),
            ],
        )
    }

    fn load_skin_shader(renderer: &Renderer) -> Shader {
        let shader = Self::compose_shader(include_str!("../shaders/skin.wgsl"));

        Shader::new(
            renderer,
            &shader,
            "vs_main",
            "fs_main",
            &[
                ("Mvp", ShaderBinding::new(ShaderStage::Vertex, 0, ShaderBindingType::UniformBuffer)),
                (
                    "BoneTransformsUniform",
                    ShaderBinding::new(ShaderStage::Vertex, 1, ShaderBindingType::UniformBuffer),
                ),
                ("Sampler", ShaderBinding::new(ShaderStage::Fragment, 10, ShaderBindingType::Sampler)),
                ("Normal", ShaderBinding::new(ShaderStage::Fragment, 11, ShaderBindingType::Texture2D)),
                ("Diffuse", ShaderBinding::new(ShaderStage::Fragment, 12, ShaderBindingType::Texture2D)),
            ],
            &[
                ("Position", 0),
                ("BoneWeight", 1),
                ("BoneIndex", 2),
                ("Normal", 3),
                ("TexCoord", 4),
                ("BiTangent", 5),
                ("Color", 6),
            ],
        )
    }

    fn load_iris_shader(renderer: &Renderer) -> Shader {
        let shader = Self::compose_shader(include_str!("../shaders/iris.wgsl"));

        Shader::new(
            renderer,
            &shader,
            "vs_main",
            "fs_main",
            &[
                ("Mvp", ShaderBinding::new(ShaderStage::Vertex, 0, ShaderBindingType::UniformBuffer)),
                (
                    "BoneTransformsUniform",
                    ShaderBinding::new(ShaderStage::Vertex, 1, ShaderBindingType::UniformBuffer),
                ),
                ("Sampler", ShaderBinding::new(ShaderStage::Fragment, 10, ShaderBindingType::Sampler)),
                ("Normal", ShaderBinding::new(ShaderStage::Fragment, 11, ShaderBindingType::Texture2D)),
                ("Diffuse", ShaderBinding::new(ShaderStage::Fragment, 12, ShaderBindingType::Texture2D)),
            ],
            &[
                ("Position", 0),
                ("BoneWeight", 1),
                ("BoneIndex", 2),
                ("Normal", 3),
                ("TexCoord", 4),
                ("BiTangent", 5),
                ("Color", 6),
            ],
        )
    }

    fn load_hair_shader(renderer: &Renderer) -> Shader {
        let shader = Self::compose_shader(include_str!("../shaders/hair.wgsl"));

        Shader::new(
            renderer,
            &shader,
            "vs_main",
            "fs_main",
            &[
                ("Mvp", ShaderBinding::new(ShaderStage::Vertex, 0, ShaderBindingType::UniformBuffer)),
                (
                    "BoneTransformsUniform",
                    ShaderBinding::new(ShaderStage::Vertex, 1, ShaderBindingType::UniformBuffer),
                ),
                ("Sampler", ShaderBinding::new(ShaderStage::Fragment, 10, ShaderBindingType::Sampler)),
                ("Normal", ShaderBinding::new(ShaderStage::Fragment, 11, ShaderBindingType::Texture2D)),
                ("Diffuse", ShaderBinding::new(ShaderStage::Fragment, 12, ShaderBindingType::Texture2D)),
            ],
            &[
                ("Position", 0),
                ("BoneWeight", 1),
                ("BoneIndex", 2),
                ("Normal", 3),
                ("TexCoord", 4),
                ("BiTangent", 5),
                ("Color", 6),
            ],
        )
    }
}
