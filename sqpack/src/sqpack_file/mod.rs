use std::io;
use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::common::{decode_compressed_data, SqPackFileReference};
use crate::package::Package;

pub struct SqPackFile {
    base_dir: PathBuf,
}

impl SqPackFile {
    pub fn new(base_dir: &Path) -> io::Result<Self> {
        Ok(SqPackFile {
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

    async fn decode_file(path: &Path) -> io::Result<Vec<u8>> {
        let data = tokio::fs::read(path).await?;

        Ok(decode_compressed_data(data))
    }
}

#[async_trait]
impl Package for SqPackFile {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        let path = self.find_path(reference)?;

        Ok(Self::decode_file(&path).await?)
    }
}
