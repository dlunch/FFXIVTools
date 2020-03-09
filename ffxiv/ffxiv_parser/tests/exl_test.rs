#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::io;
    use std::iter::FromIterator;

    use sqpack_reader::{FileProviderWeb, SqPackReaderFile};

    use ffxiv_parser::ExList;
    #[tokio::test]
    async fn test_exl() -> io::Result<()> {
        let provider = FileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderFile::new(provider)?;

        let ex_list = ExList::new(&pack).await?;
        let ex_set: HashSet<String> = HashSet::from_iter(ex_list.ex_names);

        assert!(ex_set.contains("Item"));
        assert!(ex_set.contains("opening/OpeningGridania"));

        Ok(())
    }
}
