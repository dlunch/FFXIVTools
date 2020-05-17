#[cfg(test)]
mod tests {
    use ffxiv_parser::{Ex, Language};
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[tokio::test]
    async fn exd_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let ex = Ex::new(&pack, "classjob").await?;
        let languages = ex.languages();

        {
            let row = ex.index(0, languages[0]).unwrap();
            assert_eq!(row.string(1).decode(), "ADV");
            assert_eq!(row.uint8(3), 30);
            assert_eq!(row.int8(4), -1);
            assert_eq!(row.uint16(9), 100);
            assert_eq!(row.int32(28), 0);
            assert_eq!(row.bool(44), false);
            assert_eq!(row.bool(45), false);
        }

        {
            let row = ex.index(36, languages[0]).unwrap();
            assert_eq!(row.string(1).decode(), "BLU");
            assert_eq!(row.uint8(3), 31);
            assert_eq!(row.int8(4), 25);
            assert_eq!(row.uint16(9), 105);
            assert_eq!(row.int32(28), 0);
            assert_eq!(row.bool(44), true);
            assert_eq!(row.bool(45), false);
        }

        Ok(())
    }

    #[tokio::test]
    async fn exd_multi_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let ex = Ex::new(&pack, "gilshopitem").await?;

        let row = ex.index_multi(262144, 0, Language::None).unwrap();
        assert_eq!(row.int32(0), 4594);
        assert_eq!(row.bool(1), false);

        Ok(())
    }
}
