#[cfg(test)]
mod tests {
    use ffxiv_parser::Sklb;
    use havok_parser::{HavokAnimationContainer, HavokBinaryTagFileReader};
    use sqpack_reader::{ExtractedFileProviderWeb, Result, SqPackReaderExtractedFile};

    #[allow(clippy::float_cmp)]
    #[async_std::test]
    async fn skeleton_test() -> Result<()> {
        let _ = pretty_env_logger::formatted_timed_builder()
            .filter(Some("sqpack_reader"), log::LevelFilter::Debug)
            .try_init();

        let provider = ExtractedFileProviderWeb::new("https://ffxiv-data.dlunch.net/compressed/");
        let pack = SqPackReaderExtractedFile::new(provider);

        let sklb = Sklb::new(&pack, "chara/human/c0101/skeleton/base/b0001/skl_c0101b0001.sklb").await?;
        let hkx = sklb.hkx_data();

        let root = HavokBinaryTagFileReader::read(hkx);
        let raw_animation_container = root.find_object_by_type("hkaAnimationContainer");
        let animation_container = HavokAnimationContainer::new(raw_animation_container);

        let havok_skeleton = &animation_container.skeletons[0];

        assert!(havok_skeleton.bone_names.contains(&"n_root".to_owned()));
        assert_eq!(havok_skeleton.parent_indices[5], 3);
        assert_eq!(havok_skeleton.reference_pose[0].translation[0], 0.0);
        assert_eq!(havok_skeleton.reference_pose[0].scale[0], 1.0);

        Ok(())
    }
}
