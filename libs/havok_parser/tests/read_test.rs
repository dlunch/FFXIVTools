#[cfg(test)]
mod tests {
    use ffxiv_parser::Sklb;
    use havok_parser::HavokBinaryTagFileReader;
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[tokio::test]
    async fn read_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .filter(Some("havok_parser"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let sklb = Sklb::new(&pack, "chara/human/c0101/skeleton/base/b0001/skl_c0101b0001.sklb").await?;
        let hkx = sklb.hkx_data();

        let _ = HavokBinaryTagFileReader::read(hkx);

        Ok(())
    }
}
