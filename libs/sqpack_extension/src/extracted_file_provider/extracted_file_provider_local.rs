use std::path::{Path, PathBuf};

use async_trait::async_trait;
use futures::{
    FutureExt,
    stream::{FuturesUnordered, TryStreamExt},
};
use log::debug;

use sqpack::{Result, SqPackFileHash, SqPackFileReference, SqPackReaderError};

use super::ExtractedFileProvider;

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

    fn find_path(&self, hash: &SqPackFileHash) -> Result<PathBuf> {
        for path in &self.base_dirs {
            let mut path = path.clone();

            path.push(hash.folder.to_string());
            path.push(hash.file.to_string());

            if path.exists() {
                return Ok(path);
            }
        }

        debug!("No such file {}/{}", hash.folder, hash.file);
        Err(SqPackReaderError::NoSuchFile)
    }
}

#[async_trait]
impl ExtractedFileProvider for ExtractedFileProviderLocal {
    async fn read_file(&self, hash: &SqPackFileHash) -> Result<Vec<u8>> {
        let path = self.find_path(hash)?;
        debug!("Reading {}", path.to_str().unwrap());

        Ok(tokio::fs::read(path).await?)
    }

    async fn read_file_size(&self, hash: &SqPackFileHash) -> Option<u64> {
        let path = self.find_path(hash).ok()?;
        let metadata = tokio::fs::metadata(path).await;

        if let Ok(metadata) = metadata { Some(metadata.len()) } else { None }
    }

    async fn read_files(&self, references: &[&SqPackFileReference]) -> Result<Vec<(SqPackFileHash, Vec<u8>)>> {
        references
            .iter()
            .map(|&reference| self.read_file(&reference.hash).map(move |result| Ok((reference.hash, result?))))
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await
    }
}
