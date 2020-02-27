use std::io;
use std::path::{Path, PathBuf};

use async_trait::async_trait;

use super::file_provider::FileProvider;
use crate::common::SqPackFileReference;

pub struct FileProviderFile {
    base_dir: PathBuf,
}

impl FileProviderFile {
    pub fn new(base_dir: &Path) -> Box<Self> {
        Box::new(Self {
            base_dir: base_dir.to_owned(),
        })
    }

    fn find_path(&self, reference: &SqPackFileReference) -> io::Result<PathBuf> {
        let mut path = self.base_dir.clone();

        path.push(reference.folder_hash.to_string());
        path.push(reference.file_hash.to_string());

        if path.exists() {
            Ok(path)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "No such file"))
        }
    }
}

#[async_trait]
impl FileProvider for FileProviderFile {
    async fn read_file(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        let path = self.find_path(reference)?;

        tokio::fs::read(path).await
    }
}
