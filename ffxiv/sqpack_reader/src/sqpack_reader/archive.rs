use std::io;
use std::path::Path;

use bytes::Bytes;
use futures::future;
use log::debug;

use super::data::SqPackData;
use super::index::SqPackIndex;
use crate::reference::SqPackFileReference;

pub struct SqPackArchive {
    pub index: SqPackIndex,
    pub data: Vec<SqPackData>,
}

impl SqPackArchive {
    pub async fn new(index_path: &Path) -> io::Result<Self> {
        debug!("Opening {}", index_path.to_str().unwrap());

        let index_path_str = index_path.to_str().unwrap();
        let base_path = index_path_str.trim_end_matches(".index");
        let index = SqPackIndex::new(index_path).await?;

        let futures = (0..index.dat_count).map(|x| SqPackData::new(base_path, x));
        let data = future::join_all(futures).await.into_iter().collect::<io::Result<Vec<_>>>()?;

        Ok(Self { index, data })
    }

    pub async fn read_file(&self, reference: &SqPackFileReference) -> io::Result<Bytes> {
        let file_offset = self.index.find_offset(reference)?;

        let dat_index = (file_offset & 0x0f) >> 1;
        let offset = (file_offset & 0xffff_fff0) << 3;

        Ok(self.data[dat_index as usize].read(offset as u64).await?)
    }

    pub async fn read_as_compressed(&self, reference: &SqPackFileReference) -> io::Result<Bytes> {
        let file_offset = self.index.find_offset(reference)?;

        let dat_index = (file_offset & 0x0f) >> 1;
        let offset = (file_offset & 0xffff_fff0) << 3;

        Ok(self.data[dat_index as usize].read_as_compressed(offset as u64).await?)
    }
}
