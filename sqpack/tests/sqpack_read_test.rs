#[cfg(test)]
mod tests {
    use std::path::Path;

    use sqpack::Package;
    use sqpack::SqPack;
    #[tokio::test]
    async fn test_read_sqpack() {
        #[cfg(windows)]
        let pack = SqPack::new(Path::new("D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack")).unwrap();
        #[cfg(unix)]
        let pack = SqPack::new(Path::new("/mnt/d/Games/FINAL FANTASY XIV - KOREA/game/sqpack")).unwrap();

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
            assert_eq!(data.len(), 185_084);
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
