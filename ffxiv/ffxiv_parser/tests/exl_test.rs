#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::io;
    use std::iter::FromIterator;

    use ffxiv_parser::ExList;
    use sqpack_reader::{FileProviderWeb, SqPackReaderFile};

    #[tokio::test]
    async fn test_exl() -> io::Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = FileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderFile::new(provider)?;

        let ex_list = ExList::new(&pack).await?;
        let ex_set: HashSet<String> = HashSet::from_iter(ex_list.ex_names);

        assert!(ex_set.contains("Item"));
        assert!(ex_set.contains("opening/OpeningGridania"));

        Ok(())
    }
}
