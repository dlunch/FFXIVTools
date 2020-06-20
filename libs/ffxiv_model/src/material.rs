mod character_material;

use alloc::{sync::Arc, vec::Vec};

use ffxiv_parser::Mtrl;
use renderer::{Material, Renderer, Texture};

use crate::Context;

pub fn create_material(renderer: &Renderer, context: &Context, mtrl: &Mtrl, textures: &Vec<Arc<Texture>>) -> Material {
    match mtrl.shader_name() {
        "character.shpk" | "characterglass.shpk" => character_material::CharacterMaterial::create(renderer, context, mtrl, textures),
        _ => panic!(),
    }
}
