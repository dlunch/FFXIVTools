use alloc::{format, string::String, sync::Arc};

use futures::{
    FutureExt,
    stream::{FuturesUnordered, TryStreamExt},
};
use glam::Mat4;
use hashbrown::HashMap;

use eng::render::{Renderer, Texture, TextureFormat};
use ffxiv_parser::{Eqdp, Pbd, Stm};
use sqpack::{Package, Result};

use crate::constants::{BodyId, ModelPart};
use crate::shader_holder::ShaderHolder;
use crate::texture_cache::TextureCache;

pub struct Context {
    pub(crate) shader_holder: ShaderHolder,
    pub(crate) texture_cache: TextureCache,
    pub(crate) empty_texture: Arc<Texture>,
    pub(crate) staining_template: Stm,
    equipment_deformer_parameters: HashMap<BodyId, Eqdp>,
    prebone_deformer: Pbd,
}

impl Context {
    pub async fn new(renderer: &Renderer, package: &dyn Package) -> Result<Self> {
        let empty_texture = Self::create_empty_texture(renderer).await;
        let equipment_deformer_parameters = Self::create_equipment_deformer_parameters(package).await?;
        let prebone_deformer = Pbd::new(package).await?;
        let staining_template = Stm::new(package).await?;

        Ok(Self {
            shader_holder: ShaderHolder::new(renderer),
            texture_cache: TextureCache::new(),
            empty_texture,
            staining_template,
            equipment_deformer_parameters,
            prebone_deformer,
        })
    }

    pub fn get_body_deform_matrices(&self, from_id: BodyId, to_id: BodyId) -> HashMap<String, Mat4> {
        self.prebone_deformer.get_deform_matrices(from_id as u16, to_id as u16)
    }

    pub fn get_deformed_body_id(&self, body_id: BodyId, model_id: u16, model_part: ModelPart) -> BodyId {
        if body_id == BodyId::MidlanderMale {
            return BodyId::MidlanderMale;
        }

        let eqdp = self.equipment_deformer_parameters.get(&body_id).unwrap();
        if eqdp.has_model(model_id, model_part as u8) {
            body_id
        } else {
            if body_id == BodyId::MidlanderFemale {
                return BodyId::MidlanderMale;
            }

            let search_id = if body_id == BodyId::LalafellFemale {
                BodyId::LalafellMale
            } else if body_id.is_child() {
                BodyId::ChildHyurMale
            } else if body_id == BodyId::HrothgarMale {
                BodyId::RoegadynMale
            } else if body_id.is_male() {
                BodyId::MidlanderMale
            } else {
                BodyId::MidlanderFemale
            };

            let eqdp = self.equipment_deformer_parameters.get(&search_id).unwrap();
            if eqdp.has_model(model_id, model_part as u8) {
                return search_id;
            }
            BodyId::MidlanderMale
        }
    }

    async fn create_empty_texture(renderer: &Renderer) -> Arc<Texture> {
        Arc::new(Texture::with_texels(renderer, 1, 1, &[0, 0, 0, 0], TextureFormat::Rgba8Unorm))
    }

    async fn create_equipment_deformer_parameters(package: &dyn Package) -> Result<HashMap<BodyId, Eqdp>> {
        enum_iterator::all::<BodyId>()
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
