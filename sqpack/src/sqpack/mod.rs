mod archive;
mod ext;
mod index;
mod parser;
mod reference;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::package::Package;

use self::archive::SqPackArchiveId;
use self::index::SqPackIndex;
use self::reference::SqPackFileReference;

struct SqPackArchive {
    pub index: SqPackIndex,
    pub data: Vec<File>,
}

pub struct SqPack {
    archives: HashMap<SqPackArchiveId, SqPackArchive>,
}

impl SqPack {
    pub fn new() -> SqPack {
        SqPack {
            archives: HashMap::new(),
        }
    }

    pub fn mount(&mut self, path: &Path) -> io::Result<()> {
        let path_str = path.to_str().unwrap();
        let index_path = format!("{}.index", path_str);
        let index = SqPackIndex::new(Path::new(&index_path))?;

        let data = (0..index.dat_count)
            .map(|x| File::open(format!("{}.dat{}", path_str, x)))
            .collect::<Result<Vec<_>, _>>()?;
        self.archives
            .insert(SqPack::get_archive_id(path), SqPackArchive { index, data });

        Ok(())
    }

    fn do_read_file(&self, reference: &SqPackFileReference) -> io::Result<Vec<u8>> {
        let archive = self
            .archives
            .get(&reference.archive_id)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No such archive"))?;

        let file_offset = archive.index.find_offset(reference)?;

        let dat_index = (file_offset & 0x0f) >> 1;
        let offset = (file_offset & 0xffff_fff0) << 3;

        Ok(Vec::new())
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
    fn read_file(&self, path: &str) -> io::Result<Vec<u8>> {
        self.do_read_file(&SqPackFileReference::new(path))
    }
}

#[cfg(test)]
mod tests {
    use super::SqPack;
    use crate::package::Package;
    use std::path::Path;
    #[test]
    fn test_read() {
        let mut pack = SqPack::new();
        pack.mount(Path::new(
            "D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack\\ffxiv\\0a0000.win32",
        ))
        .unwrap();
        pack.mount(Path::new(
            "D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack\\ex1\\020101.win32",
        ))
        .unwrap();

        pack.read_file("exd/item.exh").unwrap();
        pack.read_file("bg/ex1/01_roc_r2/common/bgparts/r200_a0_bari1.mdl")
            .unwrap();
    }
}
