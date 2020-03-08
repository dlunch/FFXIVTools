#[cfg(test)]
mod tests {
    use sqpack_reader::{FileProviderWeb, Package, SqPackReaderFile};
    use std::io;

    #[cfg(feature = "test_local")]
    #[tokio::test]
    #[cfg(unix)]
    async fn read_test_file() -> io::Result<()> {
        use std::path::Path;

        use sqpack_reader::FileProviderFile;

        {
            let provider = FileProviderFile::new(Path::new("/mnt/i/FFXIVData/data/kor_505"));
            let pack = SqPackReaderFile::new(provider)?;

            let data = pack.read_file("exd/item.exh").await?;
            assert_eq!(data[0], b'E');
            assert_eq!(data[1], b'X');
            assert_eq!(data[2], b'H');
            assert_eq!(data[3], b'F');
            assert_eq!(data.len(), 854);
        }

        {
            let provider = FileProviderFile::new(Path::new("/mnt/i/FFXIVData/data/kor_500"));
            let pack = SqPackReaderFile::new(provider)?;

            let data = pack.read_file("chara/accessory/a0001/model/c0101a0001_ear.mdl").await?;
            assert_eq!(data[0], 3u8);
            assert_eq!(data.len(), 27_284);
        }

        {
            let provider = FileProviderFile::new(Path::new("/mnt/i/FFXIVData/data/kor_500"));
            let pack = SqPackReaderFile::new(provider)?;

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
    async fn read_test_web() -> io::Result<()> {
        let provider = FileProviderWeb::new("https://ffxiv-data3.dlunch.net/compressed/");
        let pack = SqPackReaderFile::new(provider)?;
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
            assert_eq!(data.len(), 27_284);
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
