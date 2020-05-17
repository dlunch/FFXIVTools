#[cfg(test)]
mod tests {
    use ffxiv_parser::{Tex, TextureType};
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[tokio::test]
    async fn tex_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let tex = Tex::new(&pack, "chara/human/c0101/obj/body/b0001/texture/c0101b0001_d.tex").await?;
        assert_eq!(tex.width(), 256);
        assert_eq!(tex.height(), 512);
        assert_eq!(tex.mipmap_count(), 10);
        // assert_eq requires #[derive(Debug)]
        assert!(tex.texture_type() == TextureType::DXT1);

        Ok(())
    }
}
