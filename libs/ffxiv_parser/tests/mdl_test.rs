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
        let buffer_item = mdl.buffer_items().next().unwrap();
        assert!(buffer_item.item_type == BufferItemType::Half4);
        assert!(buffer_item.usage == BufferItemUsage::Position);

        Ok(())
    }
}
