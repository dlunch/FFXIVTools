#[cfg(test)]
#[cfg(feature = "test_local")]
mod tests {
    use std::path::Path;

    use sqpack_reader::{Package, Result, SqPackReader};

    #[tokio::test]
    async fn read_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        #[cfg(windows)]
        let pack = SqPackReader::new(Path::new("D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack"))?;
        #[cfg(unix)]
        let pack = SqPackReader::new(Path::new("/mnt/d/Games/FINAL FANTASY XIV - KOREA/game/sqpack"))?;

        {
            let data = pack.read_file("exd/item.exh").await?;
            assert_eq!(data[0], b'E');
            assert_eq!(data[1], b'X');
            assert_eq!(data[2], b'H');
            assert_eq!(data[3], b'F');
            assert_eq!(data.len(), 882);
        }

        {
            let data = pack.read_file("bg/ex1/01_roc_r2/common/bgparts/r200_a0_bari1.mdl").await?;
            assert_eq!(data[0], 3u8);
            assert_eq!(data.len(), 185_088);
        }

        {
            let data = pack.read_file("common/graphics/texture/dummy.tex").await?;
            assert_eq!(data[0], 0u8);
            assert_eq!(data[1], 0u8);
            assert_eq!(data[2], 128u8);
            assert_eq!(data[3], 0u8);
            assert_eq!(data.len(), 104);
        }

        Ok(())
    }
}
