#[cfg(feature = "test_local")]
#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;

    use sqpack_reader::{ExtractedFileProviderLocal, Package, SqPackReader, SqPackReaderExtractedFile};

    #[tokio::test]
    #[cfg(unix)]
    async fn read_as_compressed_test() -> io::Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();
        {
            #[cfg(windows)]
            let pack = SqPackReader::new(Path::new("D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack"))?;
            #[cfg(unix)]
            let pack = SqPackReader::new(Path::new("/mnt/d/Games/FINAL FANTASY XIV - KOREA/game/sqpack"))?;
            let pack_file = SqPackReaderExtractedFile::new(ExtractedFileProviderLocal::with_path(Path::new("/mnt/i/FFXIVData/data/kor_510")))?;

            let data = pack.read_as_compressed("exd/item.exh").await?;
            let data_file = pack_file.read_as_compressed("exd/item.exh").await?;

            assert_eq!(data, data_file);
        }

        Ok(())
    }
}
