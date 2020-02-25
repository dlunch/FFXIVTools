#[cfg(test)]
mod tests {
    use sqpack::Package;
    use sqpack::SqPackFile;
    use std::path::Path;
    #[tokio::test]
    #[cfg(unix)]
    async fn test_read() {
        #[cfg(unix)]
        let pack = SqPackFile::new(Path::new("/mnt/i/FFXIVData/data")).unwrap();

        {
            let data = pack.read_file("exd/item.exh").await.unwrap();
            assert_eq!(data[0], b'E');
            assert_eq!(data[1], b'X');
            assert_eq!(data[2], b'H');
            assert_eq!(data[3], b'F');
            assert_eq!(data.len(), 854);
        }

        {
            let data = pack.read_file("bg/ex1/01_roc_r2/common/bgparts/r200_a0_bari1.mdl").await.unwrap();
            assert_eq!(data[0], 3u8);
            assert_eq!(data.len(), 185_088);
        }

        {
            let data = pack.read_file("common/graphics/texture/dummy.tex").await.unwrap();
            assert_eq!(data[0], 0u8);
            assert_eq!(data[1], 0u8);
            assert_eq!(data[2], 128u8);
            assert_eq!(data[3], 0u8);
            assert_eq!(data.len(), 104);
        }
    }
}
