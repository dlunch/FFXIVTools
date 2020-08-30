#[cfg(test)]
mod tests {
    use ffxiv_parser::{Ex, Language};
    use sqpack::Result;
    use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

    #[async_std::test]
    async fn string_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let ex = Ex::new(&pack, "placename").await?;

        let row = ex.index(463, Language::English).unwrap();
        assert_eq!(row.string(2).decode(), "<i>Ragnarok</i>");

        Ok(())
    }
}
