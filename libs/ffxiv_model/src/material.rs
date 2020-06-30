mod character_material;
mod hair_material;
mod iris_material;
mod skin_material;

use alloc::{sync::Arc, vec, vec::Vec};
use zerocopy::AsBytes;

use hashbrown::HashMap;

use ffxiv_parser::Mtrl;
use renderer::{Material, Renderer, Texture};

use crate::Context;

pub async fn create_material(renderer: &Renderer, context: &Context, mtrl: &Mtrl, textures: &[Arc<Texture>]) -> Material {
    // TODO temp
    let bone_transforms = vec![1.0f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0.]
        .into_iter()
        .cycle()
        .take(4 * 3 * 64)
        .collect::<Vec<_>>();
    let bone_transform_buffer = renderer.buffer_pool.alloc(bone_transforms.len() * core::mem::size_of::<f32>());
    bone_transform_buffer.write(bone_transforms.as_bytes()).await.unwrap();

    let mut uniforms = HashMap::new();
    uniforms.insert("BoneTransformsUniform", bone_transform_buffer);

    // we can't move textures because of https://github.com/rust-lang/rust/issues/63033
    let mut textures = gather_textures(mtrl, textures);
    match mtrl.shader_name() {
        "character.shpk" | "characterglass.shpk" => {
            character_material::CharacterMaterial::create(renderer, context, mtrl, &mut textures, &uniforms).await
        }
        "hair.shpk" => hair_material::HairMaterial::create(renderer, context, &mut textures, &uniforms),
        "iris.shpk" => iris_material::IrisMaterial::create(renderer, context, &mut textures, &uniforms),
        "skin.shpk" => skin_material::SkinMaterial::create(renderer, context, &mut textures, &uniforms),
        _ => panic!(),
    }
}

pub fn gather_textures(mtrl: &Mtrl, textures: &[Arc<Texture>]) -> HashMap<&'static str, Arc<Texture>> {
    mtrl.parameters()
        .iter()
        .map(|parameter| (parameter.parameter_type.as_str(), textures[parameter.texture_index as usize].clone()))
        .collect::<HashMap<_, _>>()
}
