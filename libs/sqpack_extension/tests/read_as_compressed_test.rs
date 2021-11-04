#![cfg(feature = "test_local")]

#[tokio::test]
#[cfg(unix)]
async fn read_as_compressed_test() -> sqpack::Result<()> {
    use std::path::Path;

    use sqpack::{ExtractedFileProviderLocal, SqPackFileReference, SqPackReader, SqPackReaderExtractedFile};

    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack"), log::LevelFilter::Debug)
        .try_init();
    {
        let pack = SqPackReader::new(Path::new("/mnt/d/Games/FINAL FANTASY XIV - KOREA/game/sqpack"))?;
        let pack_file = SqPackReaderExtractedFile::new(ExtractedFileProviderLocal::with_path(Path::new("/mnt/i/FFXIVData/data/kor_510")));

        let data = pack.read_as_compressed("exd/item.exh").await?;
        let reference = SqPackFileReference::new("exd/item.exh");
        let data_file = pack_file.read_as_compressed_by_hash(&reference.hash).await?;

        assert_eq!(data, data_file);
    }

    Ok(())
}
