#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;

    use sqpack::FileProviderWeb;
    use sqpack::SqPackFile;

    use ffxiv_parser::ExList;
    #[tokio::test]
    async fn test_exl() {
        let provider = FileProviderWeb::new("https://ffxiv-data3.dlunch.net/compressed/");
        let pack = SqPackFile::new(provider).unwrap();

        let ex_list = ExList::new(&pack).await.unwrap();
        let ex_set: HashSet<String> = HashSet::from_iter(ex_list.ex_names);

        assert!(ex_set.contains("Item"));
        assert!(ex_set.contains("opening/OpeningGridania"));
    }
}
