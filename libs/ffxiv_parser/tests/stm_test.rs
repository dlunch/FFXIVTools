#[cfg(test)]
mod tests {
    use ffxiv_parser::Stm;
    use sqpack::Result;
    use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

    #[async_std::test]
    async fn stm_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let stm = Stm::new(&pack).await?;
        let data = stm.get(100);
        assert_eq!(data[0], 0x80);
        assert_eq!(data[1], 0x01);
        assert_eq!(data[20], 0x74);

        let data = stm.get(101);
        assert_eq!(data[0], 0x80);
        assert_eq!(data[1], 0x01);
        assert_eq!(data[20], 0xc1);

        Ok(())
    }
}
