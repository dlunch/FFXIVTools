mod character_material;
mod hair_material;
mod iris_material;
mod skin_material;

use alloc::sync::Arc;

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Buffer, Material, Renderer, Texture};

use crate::context::Context;
use crate::customization::Customization;

pub fn create_material(
    renderer: &Renderer,
    context: &Context,
    mtrl: &Mtrl,
    textures: &[Arc<Texture>],
    bone_transform: Arc<Buffer>,
    #[allow(unused_variables)] customization: &Customization,
    stain_id: u8,
) -> Material {
    let mut uniforms = HashMap::new();
    uniforms.insert("BoneTransformsUniform", bone_transform);

    // we can't move textures because of https://github.com/rust-lang/rust/issues/63033
    let textures = gather_textures(mtrl, textures);
    match mtrl.shader_name() {
        "character.shpk" | "characterglass.shpk" => {
            character_material::CharacterMaterial::create(renderer, context, mtrl, stain_id, textures, uniforms)
        }
        "hair.shpk" => hair_material::HairMaterial::create(renderer, context, textures, uniforms),
        "iris.shpk" => iris_material::IrisMaterial::create(renderer, context, textures, uniforms),
        "skin.shpk" => skin_material::SkinMaterial::create(renderer, context, textures, uniforms),
        _ => panic!(),
    }
}

pub fn gather_textures(mtrl: &Mtrl, textures: &[Arc<Texture>]) -> HashMap<&'static str, Arc<Texture>> {
    mtrl.parameters()
        .iter()
        .map(|parameter| (parameter.parameter_type.as_str(), textures[parameter.texture_index as usize].clone()))
        .collect::<HashMap<_, _>>()
}
