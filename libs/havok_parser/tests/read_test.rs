#[cfg(test)]
mod tests {
    use ffxiv_parser::Sklb;
    use havok_parser::HavokBinaryTagFileReader;
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[tokio::test]
    async fn read_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let sklb = Sklb::new(&pack, "chara/human/c0101/skeleton/base/b0001/skl_c0101b0001.sklb").await?;
        let hkx = sklb.hkx_data();

        let root = HavokBinaryTagFileReader::read(hkx);
        let animation_container = root.find_object_by_type("hkaAnimationContainer");
        assert_eq!(&*animation_container.borrow().object_type.name, "hkaAnimationContainer");

        Ok(())
    }
}
