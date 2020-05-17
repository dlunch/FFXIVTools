use futures::{future, FutureExt};

use ffxiv_parser::{Mdl, Mtrl, Tex};
use sqpack_reader::{Package, Result};

use crate::constants::{BodyId, ModelPart};

pub struct ModelData {
    pub mdl: Mdl,
    pub mtrls: Vec<(Mtrl, Vec<Tex>)>,
}

pub struct ModelReader {}

impl ModelReader {
    pub async fn read_equipment(
        package: &dyn Package,
        body_id: BodyId,
        body_type: u16,
        body_variant_id: u16,
        equipment_id: u16,
        equipment_variant_id: u16,
        equipment_part: ModelPart,
    ) -> Result<ModelData> {
        let mdl_filename = format!(
            "chara/equipment/e{equipment_id:04}/model/c{body_id:04}e{equipment_id:04}_{equipment_part}.mdl",
            equipment_id = equipment_id,
            body_id = body_id as u16,
            equipment_part = equipment_part.as_path_str()
        );

        Ok(Self::read_mdl(package, &mdl_filename, |material_file| {
            Self::convert_equipment_material_filename(material_file, body_id, body_type, body_variant_id, equipment_id, equipment_variant_id)
        })
        .await?)
    }

    pub async fn read_face(package: &dyn Package, body_id: BodyId, face_id: u16) -> Result<ModelData> {
        let mdl_filename = format!(
            "chara/human/c{body_id:04}/obj/face/f{face_id:04}/model/c{body_id:04}f{face_id:04}_fac.mdl",
            body_id = body_id as u16,
            face_id = face_id
        );

        Ok(Self::read_mdl(package, &mdl_filename, |material_file| {
            format!(
                "chara/human/c{body_id:04}/obj/face/f{face_id:04}/material/mt_c{body_id:04}f{face_id:04}{path}",
                body_id = body_id as u16,
                face_id = face_id,
                path = &material_file[14..]
            )
        })
        .await?)
    }

    pub async fn read_hair(package: &dyn Package, body_id: BodyId, hair_id: u16, hair_variant_id: u16) -> Result<ModelData> {
        let mdl_filename = format!(
            "chara/human/c{body_id:04}/obj/hair/h{hair_id:04}/model/c{body_id:04}h{hair_id:04}_hir.mdl",
            body_id = body_id as u16,
            hair_id = hair_id
        );

        Ok(Self::read_mdl(package, &mdl_filename, |material_file| {
            format!(
                "chara/human/c{body_id:04}/obj/hair/h{hair_id:04}/material/v{hair_variant_id:04}/mt_c{body_id:04}h{hair_id:04}{path}",
                body_id = body_id as u16,
                hair_id = hair_id,
                hair_variant_id = hair_variant_id,
                path = &material_file[14..]
            )
        })
        .await?)
    }

    async fn read_mdl<F>(package: &dyn Package, mdl_filename: &str, material_path_fetcher: F) -> Result<ModelData>
    where
        F: Fn(&str) -> String,
    {
        let mdl = Mdl::new(package, &mdl_filename).await?;

        let mtrls = future::join_all(mdl.material_files().map(|material_file| {
            let material_file = material_path_fetcher(&material_file);
            Mtrl::new(package, material_file).then(|mtrl| async {
                let mtrl = mtrl?;
                let texs = future::join_all(mtrl.texture_files().map(|texture_file| Tex::new(package, texture_file)))
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>>>()?;

                Ok((mtrl, texs))
            })
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

        Ok(ModelData { mdl, mtrls })
    }

    fn convert_equipment_material_filename(
        material_file: &str,
        body_id: BodyId,
        body_type: u16,
        body_variant_id: u16,
        equipment_id: u16,
        equipment_variant_id: u16,
    ) -> String {
        if material_file.chars().nth(9).unwrap() == 'b' {
            format!(
                "chara/human/c{body_id:04}/obj/body/b{body_type:04}/material/v{variant_id:04}/mt_c{body_id:04}b{body_type:04}{path}",
                body_id = body_id as u16,
                body_type = body_type,
                variant_id = body_variant_id,
                path = &material_file[14..]
            )
        } else {
            format!(
                "chara/equipment/e{:04}/material/v{:04}{}",
                equipment_id, equipment_variant_id, material_file
            )
        }
    }
}
