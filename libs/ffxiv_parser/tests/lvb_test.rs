use ffxiv_parser::Lvb;
use sqpack::Result;
use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

#[tokio::test]
async fn lvb_test() -> Result<()> {
    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack"), log::LevelFilter::Debug)
        .try_init();

    let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
    let pack = SqPackReaderExtractedFile::new(provider);

    let lvb = Lvb::new(&pack, "bg/ffxiv/sea_s1/twn/s1t1/level/s1t1.lvb").await?;
    assert!(lvb.lgb_paths.iter().any(|x| x == "bg/ffxiv/sea_s1/twn/s1t1/level/bg.lgb"));

    Ok(())
}
