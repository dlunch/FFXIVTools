use alloc::{format, string::String, sync::Arc, vec::Vec};

use futures::{future, FutureExt};

use eng::render::{Renderer, Texture};
use ffxiv_parser::{Mdl, Mtrl};
use sqpack::{Package, Result, SqPackReaderError};

use crate::constants::{BodyId, ModelPart};
use crate::context::Context;
use crate::customization::Customization;
use crate::equipment::Equipment;

pub struct ModelData {
    pub mdl: Mdl,
    pub mtrls: Vec<(Mtrl, Vec<Arc<Texture>>)>,
}

pub struct EquipmentModelData {
    pub model_data: ModelData,
    pub original_body_id: BodyId,
    pub deformed_body_id: BodyId,
    pub stain_id: u8,
}

pub struct ModelReader {}

impl ModelReader {
    pub async fn read_equipment(
        renderer: &Renderer,
        package: &dyn Package,
        customization: &Customization,
        equipment_part: ModelPart,
        equipment: Equipment,
        context: &Context,
    ) -> Result<EquipmentModelData> {
        let deformed_body_id = context.get_deformed_body_id(customization.body_id, equipment.model_id, equipment_part);

        let mdl_path = format!(
            "chara/equipment/e{equipment_id:04}/model/c{body_id:04}e{equipment_id:04}_{equipment_part}.mdl",
            equipment_id = equipment.model_id,
            body_id = deformed_body_id as u16,
            equipment_part = equipment_part.as_path_str()
        );

        let model_data = Self::read_mdl(renderer, package, &mdl_path, context, |material_path| {
            Self::convert_equipment_material_path(material_path, customization, equipment.model_id, equipment.variant_id)
        })
        .await?;

        Ok(EquipmentModelData {
            model_data,
            original_body_id: customization.body_id,
            deformed_body_id,
            stain_id: equipment.stain_id,
        })
    }

    pub async fn read_face(renderer: &Renderer, package: &dyn Package, customization: &Customization, context: &Context) -> Result<ModelData> {
        let mdl_path = format!(
            "chara/human/c{body_id:04}/obj/face/f{face_id:04}/model/c{body_id:04}f{face_id:04}_fac.mdl",
            body_id = customization.body_id as u16,
            face_id = customization.face_id
        );

        Self::read_mdl(renderer, package, &mdl_path, context, |material_path| {
            format!(
                "chara/human/c{body_id:04}/obj/face/f{face_id:04}/material/mt_c{body_id:04}f{face_id:04}{path}",
                body_id = customization.body_id as u16,
                face_id = customization.face_id,
                path = &material_path[14..]
            )
        })
        .await
    }

    pub async fn read_hair(renderer: &Renderer, package: &dyn Package, customization: &Customization, context: &Context) -> Result<ModelData> {
        let mdl_path = format!(
            "chara/human/c{body_id:04}/obj/hair/h{hair_id:04}/model/c{body_id:04}h{hair_id:04}_hir.mdl",
            body_id = customization.body_id as u16,
            hair_id = customization.hair_id
        );

        Self::read_mdl(renderer, package, &mdl_path, context, |material_path| {
            format!(
                "chara/human/c{body_id:04}/obj/hair/h{hair_id:04}/material/v{hair_variant_id:04}/mt_c{body_id:04}h{hair_id:04}{path}",
                body_id = customization.body_id as u16,
                hair_id = customization.hair_id,
                hair_variant_id = customization.hair_variant_id,
                path = &material_path[14..]
            )
        })
        .await
    }

    async fn read_mdl<F>(renderer: &Renderer, package: &dyn Package, mdl_path: &str, context: &Context, material_path_fetcher: F) -> Result<ModelData>
    where
        F: Fn(&str) -> String,
    {
        let mdl = Mdl::new(package, &mdl_path).await?;

        let mtrls = future::try_join_all(mdl.material_paths().map(|material_path| {
            let material_path = material_path_fetcher(material_path);
            Mtrl::new(package, material_path).then(|mtrl| async {
                let mtrl = mtrl?;
                let texs = future::try_join_all(
                    mtrl.texture_paths()
                        .map(|texture_path| context.texture_cache.get_or_read(renderer, package, texture_path)),
                )
                .await?;

                Ok::<_, SqPackReaderError>((mtrl, texs))
            })
        }))
        .await?;

        Ok(ModelData { mdl, mtrls })
    }

    fn convert_equipment_material_path(material_path: &str, customization: &Customization, equipment_id: u16, equipment_variant_id: u8) -> String {
        if material_path.chars().nth(9).unwrap() == 'b' {
            format!(
                "chara/human/c{body_id:04}/obj/body/b{body_type:04}/material/v{variant_id:04}/mt_c{body_id:04}b{body_type:04}{path}",
                body_id = customization.body_id as u16,
                body_type = customization.body_type,
                variant_id = customization.body_variant_id,
                path = &material_path[14..]
            )
        } else {
            format!(
                "chara/equipment/e{:04}/material/v{:04}{}",
                equipment_id, equipment_variant_id, material_path
            )
        }
    }
}
