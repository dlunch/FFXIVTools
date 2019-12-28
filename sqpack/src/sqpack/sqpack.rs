use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom};
use std::path::Path;

use crate::package::Package;

use super::parser::*;

#[derive(Eq, PartialEq, Hash, Default)]
struct SqPackArchiveId {
    root: u8,
    ex: u8,
    part: u8,
}

#[derive(Default)]
struct SqPackArchive {
    folder_segment: Vec<u8>,
    file_segment: Vec<u8>,
    data: Vec<File>,
}

pub struct SqPack {
    archives: HashMap<SqPackArchiveId, SqPackArchive>,
}

impl SqPack {
    #[allow(clippy::new_without_default)]
    pub fn new() -> SqPack {
        SqPack {
            archives: HashMap::new(),
        }
    }

    pub fn mount(&mut self, path: &Path) -> Result<()> {
        let path_str = path.to_str().unwrap();
        let index_path = format!("{}.index", path_str);
        let mut f = File::open(index_path)?;

        let sqpack_header = parse!(f, SqPackHeader);
        let index_header = parse!(f, SqPackIndexSegmentHeader, sqpack_header.header_length);

        let mut folder_segment = vec![0; index_header.folder_segment.size as usize];
        f.seek(SeekFrom::Start(index_header.folder_segment.offset as u64))?;
        f.read_exact(folder_segment.as_mut_slice())?;

        let mut file_segment = vec![0; index_header.file_segment.size as usize];
        f.seek(SeekFrom::Start(index_header.file_segment.offset as u64))?;
        f.read_exact(file_segment.as_mut_slice())?;

        let mut data = Vec::with_capacity(index_header.dat_count as usize);
        for i in 0..index_header.dat_count {
            let dat_path = format!("{}.dat{}", path_str, i);
            data.push(File::open(dat_path)?);
        }

        self.archives.insert(
            SqPack::get_archive_id(path),
            SqPackArchive {
                folder_segment,
                file_segment,
                data,
            },
        );

        Ok(())
    }

    fn get_archive_id(path: &Path) -> SqPackArchiveId {
        let file_name = path.file_stem().unwrap().to_str().unwrap();
        let archive_id = u32::from_str_radix(file_name, 16).unwrap();

        // ex) 0a0000
        let root = (archive_id / 0x10000) as u8;
        let ex = ((archive_id / 0x100) & 0xff) as u8;
        let part = (archive_id & 0xff) as u8;

        SqPackArchiveId { root, ex, part }
    }
}

impl Package for SqPack {
    fn read_file(&self, filename: &Path) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::SqPack;
    use std::path::Path;
    #[test]
    fn test_read() {
        let path =
            Path::new("D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack\\ffxiv\\0a0000.win32");
        let mut pack = SqPack::new();
        pack.mount(path).unwrap();
    }
}
