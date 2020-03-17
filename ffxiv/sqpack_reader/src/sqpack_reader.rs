mod archive;
mod archive_container;
mod data;
mod definition;
mod index;

use std::io;
use std::path::Path;

use async_trait::async_trait;

use crate::common::SqPackFileReference;
use crate::package::Package;

use self::archive_container::SqPackArchiveContainer;

pub struct SqPackReader {
    archives: SqPackArchiveContainer,
}

impl SqPackReader {
    pub fn new(base_dir: &Path) -> io::Result<Self> {
        Ok(Self {
            archives: SqPackArchiveContainer::new(base_dir)?,
        })
    }
}

#[async_trait]
impl Package for SqPackReader {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        let result = self.archives.get_archive(reference.archive_id).await;

        result.unwrap().read_file(reference).await
    }
}
