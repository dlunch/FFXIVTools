#[cfg(test)]
mod tests {
    use sqpack_reader::{FileProviderWeb, SqPackReaderFile};
    use std::io;

    use ffxiv_parser::Ex;
    #[tokio::test]
    async fn test_exd() -> io::Result<()> {
        let provider = FileProviderWeb::new("https://ffxiv-data3.dlunch.net/compressed/");
        let pack = SqPackReaderFile::new(provider)?;

        let _ex = Ex::new(&pack, "classjob").await?;

        Ok(())
    }
}
