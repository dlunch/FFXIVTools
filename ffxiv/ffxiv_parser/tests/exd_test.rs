#[cfg(test)]
mod tests {
    use std::io;

    use ffxiv_parser::Ex;
    use sqpack_reader::{FileProviderWeb, SqPackReaderFile};

    #[tokio::test]
    async fn test_exd() -> io::Result<()> {
        pretty_env_logger::formatted_timed_builder().filter_level(log::LevelFilter::Info).init();

        let provider = FileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderFile::new(provider)?;

        let ex = Ex::new(&pack, "classjob").await?;
        let languages = ex.languages();

        let row = ex.find_row(0, languages[0]).unwrap();
        assert_eq!(row.string(1), "ADV");

        Ok(())
    }
}
