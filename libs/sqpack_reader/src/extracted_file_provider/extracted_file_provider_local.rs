use std::io;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use bytes::Bytes;
use log::debug;

use super::ExtractedFileProvider;
use crate::reference::SqPackFileHash;

pub struct ExtractedFileProviderLocal {
    base_dirs: Vec<PathBuf>,
}

impl ExtractedFileProviderLocal {
    pub fn with_paths(base_dirs: Vec<PathBuf>) -> Self {
        Self { base_dirs }
    }

    pub fn with_path(base_dir: &Path) -> Self {
        Self {
            base_dirs: vec![base_dir.to_owned()],
        }
    }

    fn find_path(&self, hash: &SqPackFileHash) -> io::Result<PathBuf> {
        for path in &self.base_dirs {
            let mut path = path.clone();

            path.push(hash.folder.to_string());
            path.push(hash.file.to_string());

            if path.exists() {
                return Ok(path);
            }
        }

        debug!("No such file {}/{}", hash.folder, hash.file);
        Err(io::Error::new(io::ErrorKind::NotFound, "No such file"))
    }
}

#[async_trait]
impl ExtractedFileProvider for ExtractedFileProviderLocal {
    async fn read_file(&self, hash: &SqPackFileHash) -> io::Result<Bytes> {
        let path = self.find_path(hash)?;
        debug!("Reading {}", path.to_str().unwrap());

        Ok(Bytes::from(tokio::fs::read(path).await?))
    }

    async fn read_file_size(&self, hash: &SqPackFileHash) -> Option<u64> {
        let path = self.find_path(hash).ok()?;
        let metadata = tokio::fs::metadata(path).await;

        if let Ok(metadata) = metadata {
            Some(metadata.len())
        } else {
            None
        }
    }
}
