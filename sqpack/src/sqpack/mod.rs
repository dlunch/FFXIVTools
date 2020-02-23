#[macro_use]
mod ext;

mod archive;
mod archive_container;
mod archive_id;
mod data;
mod definition;
mod index;
mod reference;

use async_trait::async_trait;
use std::io;
use std::path::Path;

use crate::package::Package;

use self::archive_container::SqPackArchiveContainer;
use self::reference::SqPackFileReference;

pub struct SqPack {
    archives: SqPackArchiveContainer,
}

impl SqPack {
    pub fn new(base_dir: &Path) -> io::Result<Self> {
        Ok(Self {
            archives: SqPackArchiveContainer::new(base_dir)?,
        })
    }
}

#[async_trait]
impl Package for SqPack {
    async fn read_file(&mut self, path: &str) -> io::Result<Vec<u8>> {
        let reference = SqPackFileReference::new(path);
        let archive = self.archives.get_archive(reference.archive_id).await?;

        archive.read_file(&SqPackFileReference::new(path)).await
    }
}
