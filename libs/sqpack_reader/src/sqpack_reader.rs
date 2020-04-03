mod archive;
mod archive_container;
mod data;
mod definition;
mod index;

use std::io;
use std::path::Path;

use async_trait::async_trait;
use bytes::Bytes;

use crate::archive_id::SqPackArchiveId;
use crate::package::Package;
use crate::reference::SqPackFileReference;

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

    pub async fn folders(&self, archive_id: SqPackArchiveId) -> io::Result<Vec<u32>> {
        let archive = self.archives.get_archive(archive_id).await?;

        Ok(archive.folders().collect::<Vec<_>>())
    }

    pub async fn files(&self, archive_id: SqPackArchiveId, folder_hash: u32) -> io::Result<Vec<u32>> {
        let archive = self.archives.get_archive(archive_id).await?;
        let files = archive.files(folder_hash)?;

        Ok(files.collect::<Vec<_>>())
    }

    pub async fn read_as_compressed_by_archive(&self, archive_id: SqPackArchiveId, folder_hash: u32, file_hash: u32) -> io::Result<Bytes> {
        let archive = self.archives.get_archive(archive_id).await?;

        archive.read_as_compressed(folder_hash, file_hash).await
    }
}

#[async_trait]
impl Package for SqPackReader {
    async fn read_file_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Bytes> {
        let archive = self.archives.get_archive(reference.archive_id).await?;

        archive.read_file(reference.hash.folder, reference.hash.file).await
    }

    async fn read_as_compressed_by_reference(&self, reference: &SqPackFileReference) -> io::Result<Bytes> {
        self.read_as_compressed_by_archive(reference.archive_id, reference.hash.folder, reference.hash.file)
            .await
    }
}
