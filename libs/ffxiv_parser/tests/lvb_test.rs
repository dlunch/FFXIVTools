#[cfg(test)]
mod tests {
    use ffxiv_parser::Lvb;
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[tokio::test]
    async fn lvb_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let lvb = Lvb::new(&pack, "ffxiv/sea_s1/twn/s1t1/level/s1t1").await?;
        assert!(lvb.lgb_paths.iter().any(|x| x == "bg/ffxiv/sea_s1/twn/s1t1/level/bg.lgb"));

        Ok(())
    }
}
