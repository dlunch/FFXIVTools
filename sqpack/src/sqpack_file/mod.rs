use async_trait::async_trait;
use std::io;
use std::path::{Path, PathBuf};

use crate::common::SqPackFileReference;
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
}

#[async_trait]
impl Package for SqPackFile {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}
