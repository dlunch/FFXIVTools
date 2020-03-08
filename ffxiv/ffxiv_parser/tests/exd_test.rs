#[cfg(test)]
mod tests {
    use sqpack_reader::{FileProviderWeb, SqPackReaderFile};

    use ffxiv_parser::Ex;
    #[tokio::test]
    async fn test_exd() {
        let provider = FileProviderWeb::new("https://ffxiv-data3.dlunch.net/compressed/");
        let pack = SqPackReaderFile::new(provider).unwrap();

        let _ex = Ex::new(&pack, "classjob").await.unwrap();
    }
}
