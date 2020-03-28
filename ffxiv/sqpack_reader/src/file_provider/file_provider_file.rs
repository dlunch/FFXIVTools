use std::io;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use bytes::Bytes;
use log::debug;

use super::FileProvider;
use crate::reference::SqPackFileReference;

pub struct FileProviderFile {
    base_dirs: Vec<PathBuf>,
}

impl FileProviderFile {
    pub fn with_paths(base_dirs: Vec<PathBuf>) -> Self {
        Self { base_dirs }
    }

    pub fn with_path(base_dir: &Path) -> Self {
        Self {
            base_dirs: vec![base_dir.to_owned()],
        }
    }

    fn find_path(&self, reference: &SqPackFileReference) -> io::Result<PathBuf> {
        for path in &self.base_dirs {
            let mut path = path.clone();

            path.push(reference.folder_hash.to_string());
            path.push(reference.file_hash.to_string());

            if path.exists() {
                return Ok(path);
            }
        }

        debug!("No such file {}/{}", reference.folder_hash, reference.file_hash);
        Err(io::Error::new(io::ErrorKind::NotFound, "No such file"))
    }
}

#[async_trait]
impl FileProvider for FileProviderFile {
    async fn read_file(&self, reference: &SqPackFileReference) -> io::Result<Bytes> {
        let path = self.find_path(reference)?;
        debug!("Reading {}", path.to_str().unwrap());

        Ok(Bytes::from(tokio::fs::read(path).await?))
    }
}
