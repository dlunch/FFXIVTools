use futures::{future, FutureExt};

use ffxiv_parser::{Mdl, Mtrl, Tex};
use sqpack_reader::{Package, Result};

use crate::ModelPart;

pub struct ModelReadContext {
    pub mdl: Mdl,
    pub mtrls: Vec<(Mtrl, Vec<Tex>)>,
}

impl ModelReadContext {
    pub async fn read_equipment(pack: &dyn Package, model_id: u16, body_id: u16, model_part: ModelPart) -> Result<Self> {
        let mdl_filename = format!(
            "chara/equipment/e{model_id:04}/model/c{body_id:04}e{model_id:04}_{model_part}.mdl",
            model_id = model_id,
            body_id = body_id,
            model_part = model_part.as_str()
        );
        let mdl = Mdl::new(pack, &mdl_filename).await?;

        let mtrls = future::join_all(mdl.material_files().map(|material_file| {
            let material_file = Self::convert_equipment_material_filename(&material_file);
            Mtrl::new(pack, material_file).then(|mtrl| async {
                let mtrl = mtrl?;
                let texs = future::join_all(mtrl.texture_files().map(|texture_file| Tex::new(pack, texture_file)))
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>>>()?;

                Ok((mtrl, texs))
            })
        }))
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

        Ok(Self { mdl, mtrls })
    }

    fn convert_equipment_material_filename(material_file: &str) -> String {
        if material_file.chars().nth(9).unwrap() == 'b' {
            let body_id = 201;
            let body_type = 1;
            let variant_id = 1;
            format!(
                "chara/human/c{body_id:04}/obj/body/b{body_type:04}/material/v{variant_id:04}/mt_c{body_id:04}b{body_type:04}{path}",
                body_id = body_id,
                body_type = body_type,
                variant_id = variant_id,
                path = &material_file[14..]
            )
        } else {
            let variant_id = 1; // TODO
            let equipment_id = 6016;

            format!("chara/equipment/e{:04}/material/v{:04}{}", equipment_id, variant_id, material_file)
        }
    }
}
