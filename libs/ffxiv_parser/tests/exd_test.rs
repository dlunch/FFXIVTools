use ffxiv_parser::{Ex, Language};
use sqpack::Result;
use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

#[tokio::test]
async fn exd_test() -> Result<()> {
    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack"), log::LevelFilter::Debug)
        .try_init();

    let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
    let pack = SqPackReaderExtractedFile::new(provider);

    {
        let ex = Ex::new(&pack, "classjob").await?;
        let languages = ex.languages();

        {
            let row = ex.index(0, languages[0]).unwrap();
            assert_eq!(row.string(1).decode(), "ADV");
            assert_eq!(row.uint8(3), 30);
            assert_eq!(row.int8(4), -1);
            assert_eq!(row.uint16(9), 100);
            assert_eq!(row.int32(28), 0);
            assert!(!row.bool(45));
            assert!(!row.bool(46));
        }

        {
            let row = ex.index(36, languages[0]).unwrap();
            assert_eq!(row.string(1).decode(), "BLU");
            assert_eq!(row.uint8(3), 31);
            assert_eq!(row.int8(4), 25);
            assert_eq!(row.uint16(9), 105);
            assert_eq!(row.int32(28), 0);
            assert!(row.bool(45));
            assert!(!row.bool(46));
        }
    }

    {
        let ex = Ex::new(&pack, "territorytype").await?;

        {
            let row = ex.index(128, Language::None).unwrap();
            assert!(!row.bool(11));
            assert!(row.bool(13));
            assert!(row.bool(15));
            assert!(!row.bool(16));
            assert!(!row.bool(18));
        }
    }

    Ok(())
}

#[tokio::test]
async fn exd_multi_test() -> Result<()> {
    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack"), log::LevelFilter::Debug)
        .try_init();

    let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
    let pack = SqPackReaderExtractedFile::new(provider);

    let ex = Ex::new(&pack, "gilshopitem").await?;

    let row = ex.index_multi(262144, 0, Language::None).unwrap();
    assert_eq!(row.int32(0), 4594);
    assert!(!row.bool(1));

    Ok(())
}
