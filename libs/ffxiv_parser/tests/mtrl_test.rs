#[cfg(test)]
mod tests {
    use ffxiv_parser::{Mtrl};
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[tokio::test]
    async fn mtrl_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider)?;

        {
            let mtrl = Mtrl::new(&pack, "chara/equipment/e6016/material/v0001/mt_c0201e6016_top_a.mtrl").await?;
            assert!(mtrl.texture_files().into_iter().any(|x| x == "chara/equipment/e6016/texture/v01_c0201e6016_top_n.tex"));

            let color_table = mtrl.color_table();
            assert_eq!(color_table.len(), 544);
            assert_eq!(color_table[0], 0x00u8);
            assert_eq!(color_table[1], 0x3cu8);

            assert_eq!(mtrl.shader_name(), "character.shpk");
        }

        {
            let mtrl = Mtrl::new(&pack, "chara/human/c0201/obj/body/b0001/material/v0001/mt_c0201b0001_a.mtrl").await?;
            assert!(mtrl.texture_files().into_iter().any(|x| x == "chara/human/c0201/obj/body/b0001/texture/--c0201b0001_d.tex"));
        }

        Ok(())
    }
}
