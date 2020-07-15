#[cfg(test)]
mod tests {
    use ffxiv_exd::{ClassJob, WrappedEx};
    use ffxiv_parser::{Ex, Language};
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[async_std::test]
    async fn classjob_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let raw = Ex::new(&pack, "classjob").await?;
        let ex = WrappedEx::<ClassJob>::new(raw);
        let row = ex.index(0, Language::English).unwrap();

        assert_eq!(row.name(), "adventurer");

        Ok(())
    }
}
