#[cfg(test)]
mod tests {
    use std::path::Path;

    use sqpack::Package;
    use sqpack::SqPackFile;
    #[tokio::test]
    #[cfg(unix)]
    async fn test_read_sqpack_file() {
        {
            let pack = SqPackFile::new(Path::new("/mnt/i/FFXIVData/data/kor_505")).unwrap();

            let data = pack.read_file("exd/item.exh").await.unwrap();
            assert_eq!(data[0], b'E');
            assert_eq!(data[1], b'X');
            assert_eq!(data[2], b'H');
            assert_eq!(data[3], b'F');
            assert_eq!(data.len(), 854);
        }

        {
            let pack = SqPackFile::new(Path::new("/mnt/i/FFXIVData/data/kor_500")).unwrap();

            let data = pack.read_file("chara/accessory/a0001/model/c0101a0001_ear.mdl").await.unwrap();
            assert_eq!(data[0], 3u8);
            assert_eq!(data.len(), 27_348);
        }

        {
            let pack = SqPackFile::new(Path::new("/mnt/i/FFXIVData/data/kor_500")).unwrap();

            let data = pack.read_file("chara/accessory/a0001/texture/v01_c0101a0001_ear_d.tex").await.unwrap();
            assert_eq!(data[0], 0u8);
            assert_eq!(data[1], 0u8);
            assert_eq!(data[2], 128u8);
            assert_eq!(data[3], 0u8);
            assert_eq!(data.len(), 2824);
        }
    }
}
