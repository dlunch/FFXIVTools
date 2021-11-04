use ffxiv_parser::{BufferItemType, BufferItemUsage, Mdl};
use sqpack::Result;
use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

#[tokio::test]
async fn mdl_test() -> Result<()> {
    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack"), log::LevelFilter::Debug)
        .try_init();

    let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
    let pack = SqPackReaderExtractedFile::new(provider);

    let mdl = Mdl::new(&pack, "chara/equipment/e0100/model/c1101e0100_top.mdl").await?;
    let buffer_item = mdl.buffer_items(0)[0].items().next().unwrap();
    assert!(buffer_item.item_type == BufferItemType::Float3);
    assert!(buffer_item.usage == BufferItemUsage::Position);

    {
        let meshes = mdl.meshes(0).collect::<Vec<_>>();
        assert_eq!(meshes.len(), 2);
        assert_eq!(meshes[0].mesh_info.vertex_count, 5727);
        assert_eq!(meshes[0].buffers.len(), 2);
    }
    {
        let meshes = mdl.meshes(1).collect::<Vec<_>>();
        assert_eq!(meshes.len(), 2);
        assert_eq!(meshes[0].mesh_info.vertex_count, 3307);
        assert_eq!(meshes[0].buffers.len(), 2);
    }

    {
        let meshes = mdl.meshes(2).collect::<Vec<_>>();
        assert_eq!(meshes.len(), 2);
        assert_eq!(meshes[0].mesh_info.vertex_count, 1731);
        assert_eq!(meshes[0].buffers.len(), 2);
    }

    {
        let bone_names = mdl.bone_names(0).collect::<Vec<_>>();
        assert_eq!(bone_names[0], "j_asi_a_l");
    }

    let materials = mdl.material_paths().collect::<Vec<_>>();
    assert_eq!(materials[0], "/mt_c0101e0100_top_a.mtrl");

    Ok(())
}
