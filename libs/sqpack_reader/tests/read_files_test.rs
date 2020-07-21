#[cfg(test)]
mod tests {
    #[async_std::test]
    #[cfg(feature = "std")]
    async fn read_files_test() -> sqpack_reader::Result<()> {
        use sqpack_reader::{BatchablePackage, ExtractedFileProviderWeb, SqPackFileReference, SqPackReaderExtractedFile};

        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
        let pack = SqPackReaderExtractedFile::new(provider);
        let (reference1, reference2, reference3) = (
            SqPackFileReference::new("exd/item.exh"),
            SqPackFileReference::new("chara/accessory/a0001/model/c0101a0001_ear.mdl"),
            SqPackFileReference::new("chara/accessory/a0001/texture/v01_c0101a0001_ear_d.tex"),
        );
        let references = vec![&reference1, &reference2, &reference3];
        let files = pack.read_files(&references).await?;
        {
            let data = files.get(&references[0]).unwrap();
            assert_eq!(data[0], b'E');
            assert_eq!(data[1], b'X');
            assert_eq!(data[2], b'H');
            assert_eq!(data[3], b'F');
            assert_eq!(data.len(), 904);
        }

        {
            let data = files.get(&references[1]).unwrap();
            assert_eq!(data[0], 3u8);
            assert_eq!(data.len(), 27_348);
        }

        {
            let data = files.get(&references[2]).unwrap();
            assert_eq!(data[0], 0u8);
            assert_eq!(data[1], 0u8);
            assert_eq!(data[2], 128u8);
            assert_eq!(data[3], 0u8);
            assert_eq!(data.len(), 2824);
        }

        Ok(())
    }
}
