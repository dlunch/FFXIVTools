#[cfg(test)]
mod tests {
    use ffxiv_parser::Eqdp;
    use sqpack::Result;
    use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

    #[async_std::test]
    async fn eqdp_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let eqdp = Eqdp::new(&pack, "chara/xls/charadb/equipmentdeformerparameter/c0201.eqdp").await?;
        assert!(eqdp.has_model(6016, 0));

        Ok(())
    }
}
