#[cfg(test)]
mod tests {
    use sqpack_reader::{FileProviderWeb, SqPackReaderFile};
    use std::io;

    use ffxiv_parser::Ex;
    #[tokio::test]
    async fn test_exd() -> io::Result<()> {
        let provider = FileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderFile::new(provider)?;

        let ex = Ex::new(&pack, "classjob").await?;
        let languages = ex.languages();

        let row = ex.find_row(0, languages[0]).unwrap();
        assert_eq!(row.string(1), "ADV");

        Ok(())
    }
}
