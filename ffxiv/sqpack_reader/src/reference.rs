use super::archive_id::SqPackArchiveId;

pub struct SqPackFileReference {
    pub archive_id: SqPackArchiveId,
    pub path_hash: u32,
    pub folder_hash: u32,
    pub file_hash: u32,

    #[cfg(debug_assertions)]
    pub path: String,
}

impl SqPackFileReference {
    pub fn new(path: &str) -> Self {
        let path_str = path.to_ascii_lowercase();
        let folder_separator = path_str.rfind('/').unwrap();
        let folder_str = &path_str[..folder_separator];
        let file_str = &path_str[folder_separator + 1..];

        let path_hash = !Self::hash(&path_str);
        let folder_hash = !Self::hash(&folder_str);
        let file_hash = !Self::hash(&file_str);

        Self {
            archive_id: SqPackArchiveId::with_file_path(&path_str),
            path_hash,
            folder_hash,
            file_hash,
            #[cfg(debug_assertions)]
            path: path_str,
        }
    }

    fn hash(value: &str) -> u32 {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(value.as_bytes());
        hasher.finalize()
    }
}
