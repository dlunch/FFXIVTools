mod character_material;
mod hair_material;
mod iris_material;
mod skin_material;

use alloc::{sync::Arc, vec::Vec};

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Material, Renderer, Texture};

use crate::Context;

pub fn create_material(renderer: &Renderer, context: &Context, mtrl: &Mtrl, textures: &Vec<Arc<Texture>>) -> Material {
    let textures = gather_textures(mtrl, textures);
    match mtrl.shader_name() {
        "character.shpk" | "characterglass.shpk" => character_material::CharacterMaterial::create(renderer, context, mtrl, textures),
        "hair.shpk" => hair_material::HairMaterial::create(renderer, context, mtrl, textures),
        "iris.shpk" => iris_material::IrisMaterial::create(renderer, context, mtrl, textures),
        "skin.shpk" => skin_material::SkinMaterial::create(renderer, context, mtrl, textures),
        _ => panic!(),
    }
}

pub fn gather_textures(mtrl: &Mtrl, textures: &Vec<Arc<Texture>>) -> HashMap<&'static str, Arc<Texture>> {
    mtrl.parameters()
        .iter()
        .map(|parameter| (parameter.parameter_type.as_str(), textures[parameter.texture_index as usize].clone()))
        .collect::<HashMap<_, _>>()
}
