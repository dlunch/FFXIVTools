#[cfg(test)]
mod tests {
    use ffxiv_parser::{BufferItemType, BufferItemUsage, Mdl};
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[tokio::test]
    async fn mdl_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider)?;

        let mdl = Mdl::new(&pack, "bg/ex1/01_roc_r2/common/bgparts/r200_a0_bari1.mdl").await?;
        let buffer_item = mdl.buffer_items(0).next().unwrap().items().next().unwrap();
        assert!(buffer_item.item_type == BufferItemType::Half4);
        assert!(buffer_item.usage == BufferItemUsage::Position);

        {
            let meshes = mdl.meshes(0).collect::<Vec<_>>();
            assert_eq!(meshes.len(), 1);
            assert_eq!(meshes[0].mesh_info.vertex_count, 2790);
            assert_eq!(meshes[0].buffers.len(), 2);
        }
        {
            let meshes = mdl.meshes(1).collect::<Vec<_>>();
            assert_eq!(meshes.len(), 1);
            assert_eq!(meshes[0].mesh_info.vertex_count, 1621);
            assert_eq!(meshes[0].buffers.len(), 2);
        }

        {
            let meshes = mdl.meshes(2).collect::<Vec<_>>();
            assert_eq!(meshes.len(), 1);
            assert_eq!(meshes[0].mesh_info.vertex_count, 298);
            assert_eq!(meshes[0].buffers.len(), 2);
        }

        let materials = mdl.material_files();
        assert_eq!(materials[0], "bg/ex1/01_roc_r2/common/material/r200_b0_bari1a.mtrl");

        Ok(())
    }
}
