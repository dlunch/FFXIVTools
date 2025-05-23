use ffxiv_parser::{Mtrl, MtrlParameterType};
use sqpack::Result;
use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

#[tokio::test]
async fn mtrl_test() -> Result<()> {
    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack"), log::LevelFilter::Debug)
        .try_init();

    let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
    let pack = SqPackReaderExtractedFile::new(provider);

    {
        let mtrl = Mtrl::new(&pack, "chara/equipment/e6016/material/v0001/mt_c0201e6016_top_a.mtrl").await?;
        assert!(
            mtrl.texture_paths()
                .any(|x| x == "chara/equipment/e6016/texture/v01_c0201e6016_top_n.tex")
        );

        let color_table = mtrl.color_table();
        assert_eq!(color_table.len(), 2176);
        assert_eq!(color_table[0], 0x00u8);
        assert_eq!(color_table[1], 0x3cu8);

        assert_eq!(mtrl.shader_name(), "characterlegacy.shpk");

        assert!(mtrl.parameters()[0].parameter_type == MtrlParameterType::Normal)
    }

    {
        let mtrl = Mtrl::new(&pack, "chara/human/c0201/obj/body/b0001/material/v0001/mt_c0201b0001_a.mtrl").await?;
        assert!(
            mtrl.texture_paths()
                .any(|x| x == "chara/human/c0201/obj/body/b0001/texture/c0201b0001_base.tex")
        );
    }

    Ok(())
}
