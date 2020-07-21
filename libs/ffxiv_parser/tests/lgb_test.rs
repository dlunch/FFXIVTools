#[cfg(test)]
mod tests {
    use ffxiv_parser::{LayerGroupResourceItem, Lgb};
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[async_std::test]
    async fn lgb_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let lgb = Lgb::new(&pack, "bg/ffxiv/sea_s1/twn/s1t1/level/planner.lgb").await?;
        assert_eq!(lgb.name(), "Planner");
        let entries = lgb.entries();
        match entries.get("QST_ClsAcn250_000").unwrap()[0] {
            LayerGroupResourceItem::EventNpc(x) => assert_eq!(x.item_type, 8),
            _ => panic!(),
        }

        Ok(())
    }
}
