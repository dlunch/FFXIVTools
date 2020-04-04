mod archive;
mod archive_container;
mod data;
mod definition;
mod index;

use std::io;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;

use crate::archive_id::SqPackArchiveId;
use crate::error::Result;
use crate::package::Package;
use crate::reference::SqPackFileReference;

use self::archive::SqPackArchive;
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

    pub async fn archive(&self, archive_id: SqPackArchiveId) -> io::Result<Arc<SqPackArchive>> {
        self.archives.get_archive(archive_id).await
    }
}

#[async_trait]
impl Package for SqPackReader {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> Result<Bytes> {
        let archive = self.archive(reference.archive_id).await?;

        archive.read_file(reference.hash.folder, reference.hash.file).await
    }

    async fn read_as_compressed_by_reference(&self, reference: &SqPackFileReference) -> Result<Bytes> {
        let archive = self.archive(reference.archive_id).await?;

        archive.read_as_compressed(reference.hash.folder, reference.hash.file).await
    }
}
