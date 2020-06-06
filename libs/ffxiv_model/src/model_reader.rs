use alloc::{format, string::String, sync::Arc, vec::Vec};

use futures::{future, FutureExt};

use ffxiv_parser::{Mdl, Mtrl};
use renderer::{Renderer, Texture};
use sqpack_reader::{Package, Result, SqPackReaderError};

use crate::constants::{BodyId, ModelPart};
use crate::context::Context;

pub struct ModelData {
    pub mdl: Mdl,
    pub mtrls: Vec<(Mtrl, Vec<Arc<Texture>>)>,
}

pub struct ModelReader {}

impl ModelReader {
    // TODO use customization and equipment type
    #[allow(clippy::too_many_arguments)]
    pub async fn read_equipment(
        renderer: &Renderer,
        package: &dyn Package,
        body_id: BodyId,
        body_type: u16,
        body_variant_id: u16,
        equipment_id: u16,
        equipment_variant_id: u16,
        equipment_part: ModelPart,
        context: &Context,
    ) -> Result<ModelData> {
        let mdl_path = format!(
            "chara/equipment/e{equipment_id:04}/model/c{body_id:04}e{equipment_id:04}_{equipment_part}.mdl",
            equipment_id = equipment_id,
            body_id = body_id as u16,
            equipment_part = equipment_part.as_path_str()
        );

        Ok(Self::read_mdl(renderer, package, &mdl_path, context, |material_path| {
            Self::convert_equipment_material_path(material_path, body_id, body_type, body_variant_id, equipment_id, equipment_variant_id)
        })
        .await?)
    }

    pub async fn read_face(renderer: &Renderer, package: &dyn Package, body_id: BodyId, face_id: u16, context: &Context) -> Result<ModelData> {
        let mdl_path = format!(
            "chara/human/c{body_id:04}/obj/face/f{face_id:04}/model/c{body_id:04}f{face_id:04}_fac.mdl",
            body_id = body_id as u16,
            face_id = face_id
        );

        Ok(Self::read_mdl(renderer, package, &mdl_path, context, |material_path| {
            format!(
                "chara/human/c{body_id:04}/obj/face/f{face_id:04}/material/mt_c{body_id:04}f{face_id:04}{path}",
                body_id = body_id as u16,
                face_id = face_id,
                path = &material_path[14..]
            )
        })
        .await?)
    }

    pub async fn read_hair(
        renderer: &Renderer,
        package: &dyn Package,
        body_id: BodyId,
        hair_id: u16,
        hair_variant_id: u16,
        context: &Context,
    ) -> Result<ModelData> {
        let mdl_path = format!(
            "chara/human/c{body_id:04}/obj/hair/h{hair_id:04}/model/c{body_id:04}h{hair_id:04}_hir.mdl",
            body_id = body_id as u16,
            hair_id = hair_id
        );

        Ok(Self::read_mdl(renderer, package, &mdl_path, context, |material_path| {
            format!(
                "chara/human/c{body_id:04}/obj/hair/h{hair_id:04}/material/v{hair_variant_id:04}/mt_c{body_id:04}h{hair_id:04}{path}",
                body_id = body_id as u16,
                hair_id = hair_id,
                hair_variant_id = hair_variant_id,
                path = &material_path[14..]
            )
        })
        .await?)
    }

    async fn read_mdl<F>(renderer: &Renderer, package: &dyn Package, mdl_path: &str, context: &Context, material_path_fetcher: F) -> Result<ModelData>
    where
        F: Fn(&str) -> String,
    {
        let mdl = Mdl::new(package, &mdl_path).await?;

        let mtrls = future::try_join_all(mdl.material_paths().map(|material_path| {
            let material_path = material_path_fetcher(&material_path);
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

    fn convert_equipment_material_path(
        material_path: &str,
        body_id: BodyId,
        body_type: u16,
        body_variant_id: u16,
        equipment_id: u16,
        equipment_variant_id: u16,
    ) -> String {
        if material_path.chars().nth(9).unwrap() == 'b' {
            format!(
                "chara/human/c{body_id:04}/obj/body/b{body_type:04}/material/v{variant_id:04}/mt_c{body_id:04}b{body_type:04}{path}",
                body_id = body_id as u16,
                body_type = body_type,
                variant_id = body_variant_id,
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
