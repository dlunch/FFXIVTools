#[cfg(test)]
mod tests {
    use std::io;

    use ffxiv_parser::Ex;
    use sqpack_reader::{FileProviderWeb, SqPackReaderFile};

    #[tokio::test]
    async fn test_exd() -> io::Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = FileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderFile::new(provider)?;

        let ex = Ex::new(&pack, "classjob").await?;
        let languages = ex.languages();

        let row = ex.index(0, languages[0]).unwrap();
        assert_eq!(row.string(1), "ADV");
        assert_eq!(row.uint8(3), 30);
        assert_eq!(row.int8(4), -1);
        assert_eq!(row.uint16(9), 100);
        assert_eq!(row.int32(28), 0);
        assert_eq!(row.bool(44), false);
        assert_eq!(row.bool(45), false);

        Ok(())
    }
}
