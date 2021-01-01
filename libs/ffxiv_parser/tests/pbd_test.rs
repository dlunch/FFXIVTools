use ffxiv_parser::Pbd;
use sqpack::Result;
use sqpack_extension::{ExtractedFileProviderWeb, SqPackReaderExtractedFile};

#[allow(clippy::float_cmp)]
#[async_std::test]
async fn pbd_test() -> Result<()> {
    let _ = pretty_env_logger::formatted_timed_builder()
        .filter(Some("sqpack"), log::LevelFilter::Debug)
        .try_init();

    let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/all/");
    let pack = SqPackReaderExtractedFile::new(provider);

    let pbd = Pbd::new(&pack).await?;

    let result = pbd.get_deform_matrices(101, 101); // should be empty
    assert_eq!(result.len(), 0);

    let result = pbd.get_deform_matrices(201, 101);
    assert_eq!(result["n_hara"][0], 0.9627);

    let result = pbd.get_deform_matrices(601, 101);
    assert_eq!(result["j_ago"][0], 0.89393127);

    Ok(())
}
