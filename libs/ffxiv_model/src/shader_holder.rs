use alloc::sync::Arc;

use hashbrown::HashMap;

use renderer::{Renderer, Shader, ShaderBinding, ShaderBindingType};

// hashbrown version of https://github.com/bluss/maplit/blob/master/src/lib.rs#L46
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}
#[derive(Eq, PartialEq, Hash)]
pub enum ShaderType {
    Character,
    Iris,
    Hair,
    Skin,
}

pub struct ShaderHolder {
    pub vertex_shader: Arc<Shader>,
    fragment_shaders: HashMap<ShaderType, Arc<Shader>>,
}

impl ShaderHolder {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            vertex_shader: Arc::new(Self::load_vertex_shader(renderer)),
            fragment_shaders: hashmap! {
                ShaderType::Character => Arc::new(Self::load_character_shader(renderer)),
                ShaderType::Iris => Arc::new(Self::load_iris_shader(renderer)),
                ShaderType::Hair => Arc::new(Self::load_hair_shader(renderer)),
                ShaderType::Skin => Arc::new(Self::load_skin_shader(renderer))
            },
        }
    }

    pub fn fragment_shader(&self, shader: ShaderType) -> Arc<Shader> {
        self.fragment_shaders.get(&shader).unwrap().clone()
    }

    fn load_vertex_shader(renderer: &Renderer) -> Shader {
        let vs_bytes = include_bytes!("../shaders/shader.vert.spv");

        Shader::new(
            &renderer,
            &vs_bytes[..],
            "main",
            hashmap! {"Mvp" => ShaderBinding::new(0, ShaderBindingType::UniformBuffer)},
            hashmap! {
                "Position" => 0,
                "BoneWeight" => 1,
                "BoneIndex" => 2,
                "Normal" => 3,
                "TexCoord" => 4,
                "BiTangent" => 5,
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

    fn load_iris_shader(renderer: &Renderer) -> Shader {
        let fs_bytes = include_bytes!("../shaders/iris.frag.spv");

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

    fn load_hair_shader(renderer: &Renderer) -> Shader {
        let fs_bytes = include_bytes!("../shaders/hair.frag.spv");

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
