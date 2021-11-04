use ffxiv_parser::Pap;
use sqpack::Result;
use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

#[tokio::test]
async fn pap_test() -> Result<()> {
    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack"), log::LevelFilter::Debug)
        .try_init();

    let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
    let pack = SqPackReaderExtractedFile::new(provider);

    let pap = Pap::new(&pack, "chara/human/c1101/animation/a0001/bt_common/resident/idle.pap").await?;
    let hkx = pap.hkx_data();
    assert_eq!(hkx[0], b'\x1e');
    assert_eq!(hkx[1], b'\x0d');
    assert_eq!(hkx[2], b'\xb0');
    assert_eq!(hkx[3], b'\xca');
    assert_eq!(hkx[4], b'\xce');
    assert_eq!(hkx[5], b'\xfa');

    Ok(())
}
