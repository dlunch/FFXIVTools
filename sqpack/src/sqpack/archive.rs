use std::io;
use std::path::Path;

use super::data::SqPackData;
use super::index::SqPackIndex;
use super::reference::SqPackFileReference;

pub struct SqPackArchive {
    pub index: SqPackIndex,
    pub data: Vec<SqPackData>,
}

impl SqPackArchive {
    pub async fn new(index_path: &Path) -> io::Result<Self> {
        let index_path_str = index_path.to_str().unwrap();
        let base_path = index_path_str.trim_end_matches(".index");
        let index = SqPackIndex::new(index_path).await?;

        let mut data = Vec::with_capacity(index.dat_count as usize);
        for dat_num in 0..index.dat_count {
            let path_str = format!("{}.dat{}", base_path, dat_num);
            data.push(SqPackData::new(Path::new(&path_str)).await?);
        }
        Ok(Self { index, data })
    }

    pub async fn read_file(&mut self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        let file_offset = self.index.find_offset(reference)?;

        let dat_index = (file_offset & 0x0f) >> 1;
        let offset = (file_offset & 0xffff_fff0) << 3;

        Ok(self.data[dat_index as usize].read(offset as u64).await?)
    }
}
