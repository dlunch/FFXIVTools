#[cfg(debug_assertions)]
use alloc::string::String;

use super::archive_id::SqPackArchiveId;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct SqPackFileHash {
    pub path: u32,
    pub folder: u32,
    pub file: u32,
}

impl SqPackFileHash {
    pub fn new(path_str: &str) -> Self {
        let folder_separator = path_str.rfind('/').unwrap();
        let folder_str = &path_str[..folder_separator];
        let file_str = &path_str[folder_separator + 1..];

        let path = !Self::hash(&path_str);
        let folder = !Self::hash(&folder_str);
        let file = !Self::hash(&file_str);

        Self { path, folder, file }
    }

    pub fn from_raw_hash(path_hash: u32, folder_hash: u32, file_hash: u32) -> Self {
        Self {
            path: path_hash,
            folder: folder_hash,
            file: file_hash,
        }
    }

    fn hash(value: &str) -> u32 {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(value.as_bytes());
        hasher.finalize()
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct SqPackFileReference {
    pub archive_id: SqPackArchiveId,
    pub hash: SqPackFileHash,

    #[cfg(debug_assertions)]
    pub path: String,
}

impl SqPackFileReference {
    pub fn new(path: &str) -> Self {
        let path_str = path.to_ascii_lowercase();

        Self {
            archive_id: SqPackArchiveId::from_file_path(&path_str),
            hash: SqPackFileHash::new(&path_str),
            #[cfg(debug_assertions)]
            path: path_str,
        }
    }
}
