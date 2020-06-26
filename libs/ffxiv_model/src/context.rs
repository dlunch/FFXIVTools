use alloc::{format, sync::Arc};

use enum_iterator::IntoEnumIterator;
use futures::{
    stream::{FuturesUnordered, TryStreamExt},
    FutureExt,
};
use hashbrown::HashMap;

use ffxiv_parser::Eqdp;
use renderer::{Renderer, Texture, TextureFormat};
use sqpack_reader::{Package, Result};

use crate::constants::BodyId;
use crate::shader_holder::ShaderHolder;
use crate::texture_cache::TextureCache;

pub struct Context {
    pub(crate) shader_holder: ShaderHolder,
    pub(crate) texture_cache: TextureCache,
    pub(crate) empty_texture: Arc<Texture>,
    #[allow(dead_code)]
    equipment_deformer_parameters: HashMap<BodyId, Eqdp>,
}

impl Context {
    pub async fn new(renderer: &Renderer, package: &dyn Package) -> Result<Self> {
        let empty_texture = Self::create_empty_texture(renderer).await;
        let equipment_deformer_parameters = Self::create_equipment_deformer_parameters(package).await?;

        Ok(Self {
            shader_holder: ShaderHolder::new(renderer),
            texture_cache: TextureCache::new(),
            empty_texture,
            equipment_deformer_parameters,
        })
    }

    async fn create_empty_texture(renderer: &Renderer) -> Arc<Texture> {
        Arc::new(Texture::with_texels(renderer, 1, 1, &[0, 0, 0, 0], TextureFormat::Rgba8Unorm).await)
    }

    async fn create_equipment_deformer_parameters(package: &dyn Package) -> Result<HashMap<BodyId, Eqdp>> {
        BodyId::into_enum_iter()
            .map(|body_id| {
                Eqdp::new(
                    package,
                    format!("chara/xls/charadb/equipmentdeformerparameter/c{:04}.eqdp", body_id as u16),
                )
                .map(move |eqdp| Ok((body_id, eqdp?)))
            })
            .collect::<FuturesUnordered<_>>()
            .try_collect::<HashMap<_, _>>()
            .await
    }
}
