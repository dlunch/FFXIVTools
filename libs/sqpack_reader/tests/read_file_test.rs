#[cfg(test)]
mod tests {

    #[cfg(feature = "test_local")]
    #[tokio::test]
    #[cfg(unix)]
    async fn read_file_test() -> Result<()> {
        use std::path::Path;

        use sqpack_reader::{ExtractedFileProviderLocal, Package, Result, SqPackReaderExtractedFile};

        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();
        {
            let provider = ExtractedFileProviderLocal::with_path(Path::new("/mnt/i/FFXIVData/data/kor_505"));
            let pack = SqPackReaderExtractedFile::new(provider);

            let data = pack.read_file("exd/item.exh").await?;
            assert_eq!(data[0], b'E');
            assert_eq!(data[1], b'X');
            assert_eq!(data[2], b'H');
            assert_eq!(data[3], b'F');
            assert_eq!(data.len(), 854);
        }

        {
            let provider = ExtractedFileProviderLocal::with_path(Path::new("/mnt/i/FFXIVData/data/kor_500"));
            let pack = SqPackReaderExtractedFile::new(provider);

            let data = pack.read_file("chara/accessory/a0001/model/c0101a0001_ear.mdl").await?;
            assert_eq!(data[0], 3u8);
            assert_eq!(data.len(), 27_348);
        }

        {
            let provider = ExtractedFileProviderLocal::with_path(Path::new("/mnt/i/FFXIVData/data/kor_500"));
            let pack = SqPackReaderExtractedFile::new(provider);

            let data = pack.read_file("chara/accessory/a0001/texture/v01_c0101a0001_ear_d.tex").await?;
            assert_eq!(data[0], 0u8);
            assert_eq!(data[1], 0u8);
            assert_eq!(data[2], 128u8);
            assert_eq!(data[3], 0u8);
            assert_eq!(data.len(), 2824);
        }

        Ok(())
    }

    #[tokio::test]
    #[cfg(feature = "std")]
    async fn read_web_test() -> sqpack_reader::Result<()> {
        use sqpack_reader::{ExtractedFileProviderWeb, Package, SqPackReaderExtractedFile};

        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider);
        {
            let data = pack.read_file("exd/item.exh").await?;
            assert_eq!(data[0], b'E');
            assert_eq!(data[1], b'X');
            assert_eq!(data[2], b'H');
            assert_eq!(data[3], b'F');
            assert_eq!(data.len(), 904);
        }

        {
            let data = pack.read_file("chara/accessory/a0001/model/c0101a0001_ear.mdl").await?;
            assert_eq!(data[0], 3u8);
            assert_eq!(data.len(), 27_348);
        }

        {
            let data = pack.read_file("chara/accessory/a0001/texture/v01_c0101a0001_ear_d.tex").await?;
            assert_eq!(data[0], 0u8);
            assert_eq!(data[1], 0u8);
            assert_eq!(data[2], 128u8);
            assert_eq!(data[3], 0u8);
            assert_eq!(data.len(), 2824);
        }

        Ok(())
    }
}
