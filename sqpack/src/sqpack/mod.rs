#[macro_use]
mod ext;

mod archive;
mod archive_container;
mod archive_id;
mod data;
mod definition;
mod index;
mod reference;

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

impl Package for SqPack {
    fn read_file(&mut self, path: &str) -> io::Result<Vec<u8>> {
        let reference = SqPackFileReference::new(path);
        let archive = self.archives.get_archive(&reference.archive_id);

        archive.read_file(&SqPackFileReference::new(path))
    }
}

#[cfg(test)]
mod tests {
    use super::SqPack;
    use crate::package::Package;
    use std::path::Path;
    #[test]
    fn test_read() {
        let mut pack = SqPack::new(Path::new("D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack")).unwrap();

        {
            let data = pack.read_file("exd/item.exh").unwrap();
            assert_eq!(data[0], b'E');
            assert_eq!(data[1], b'X');
            assert_eq!(data[2], b'H');
            assert_eq!(data[3], b'F');
            assert_eq!(data.len(), 854);
        }

        {
            let data = pack.read_file("bg/ex1/01_roc_r2/common/bgparts/r200_a0_bari1.mdl").unwrap();
            assert_eq!(data[0], 3u8);
            assert_eq!(data.len(), 185_088);
        }

        {
            let data = pack.read_file("common/graphics/texture/dummy.tex").unwrap();
            assert_eq!(data[0], 0u8);
            assert_eq!(data[1], 0u8);
            assert_eq!(data[2], 128u8);
            assert_eq!(data[3], 0u8);
            assert_eq!(data.len(), 104);
        }
    }
}
