mod character_material;
mod hair_material;
mod iris_material;
mod skin_material;

use alloc::sync::Arc;

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Material, Renderer, Texture};

use crate::Context;

pub async fn create_material(renderer: &Renderer, context: &Context, mtrl: &Mtrl, textures: &[Arc<Texture>]) -> Material {
    // we can't move textures because of https://github.com/rust-lang/rust/issues/63033
    let mut textures = gather_textures(mtrl, textures);
    match mtrl.shader_name() {
        "character.shpk" | "characterglass.shpk" => character_material::CharacterMaterial::create(renderer, context, mtrl, &mut textures).await,
        "hair.shpk" => hair_material::HairMaterial::create(renderer, context, mtrl, &mut textures),
        "iris.shpk" => iris_material::IrisMaterial::create(renderer, context, mtrl, &mut textures),
        "skin.shpk" => skin_material::SkinMaterial::create(renderer, context, mtrl, &mut textures),
        _ => panic!(),
    }
}

pub fn gather_textures(mtrl: &Mtrl, textures: &[Arc<Texture>]) -> HashMap<&'static str, Arc<Texture>> {
    mtrl.parameters()
        .iter()
        .map(|parameter| (parameter.parameter_type.as_str(), textures[parameter.texture_index as usize].clone()))
        .collect::<HashMap<_, _>>()
}
