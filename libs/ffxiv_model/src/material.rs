mod character_material;
mod hair_material;
mod iris_material;
mod skin_material;

use alloc::sync::Arc;

use hashbrown::HashMap;

use eng::render::{Buffer, Material, Renderer, Resource, Texture};
use ffxiv_parser::{Mtrl, MtrlParameterType};

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
    // we can't move textures because of https://github.com/rust-lang/rust/issues/63033
    let mut resources = gather_textures(mtrl, textures);
    resources.insert("bone_transform", bone_transform);

    match mtrl.shader_name() {
        "character.shpk" | "characterglass.shpk" => character_material::CharacterMaterial::create(renderer, context, mtrl, stain_id, resources),
        "hair.shpk" => hair_material::HairMaterial::create(renderer, context, resources),
        "iris.shpk" => iris_material::IrisMaterial::create(renderer, context, resources),
        "skin.shpk" => skin_material::SkinMaterial::create(renderer, context, resources),
        _ => panic!(),
    }
}

pub fn gather_textures(mtrl: &Mtrl, textures: &[Arc<Texture>]) -> HashMap<&'static str, Arc<dyn Resource>> {
    mtrl.parameters()
        .iter()
        .map(|parameter| {
            (
                parameter_type_to_shader_name(&parameter.parameter_type),
                textures[parameter.texture_index as usize].clone() as Arc<dyn Resource>,
            )
        })
        .collect::<HashMap<_, _>>()
}

fn parameter_type_to_shader_name(parameter_type: &MtrlParameterType) -> &'static str {
    match parameter_type {
        MtrlParameterType::Normal => "normal_tex",
        MtrlParameterType::Mask => "mask_tex",
        MtrlParameterType::Diffuse => "diffuse_tex",
        MtrlParameterType::Specular => "specular_tex",
        MtrlParameterType::Catchlight => "catchlight_tex",
    }
}
