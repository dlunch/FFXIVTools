#[cfg(test)]
mod tests {
    use ffxiv_ex::{ClassJob, NamedExRow, WrappedEx};
    use ffxiv_parser::Language;
    use sqpack::Result;
    use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

    #[async_std::test]
    async fn classjob_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let ex = WrappedEx::<ClassJob>::new(&pack).await?;
        let row = ex.index(0, Language::English).unwrap();

        assert_eq!(row.name(), "adventurer");

        Ok(())
    }
}
