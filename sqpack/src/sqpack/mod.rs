mod archive;
mod ext;
mod index;
mod parser;
mod reference;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

use crate::package::Package;

use self::archive::{SqPackArchive, SqPackArchiveId};
use self::reference::SqPackFileReference;

pub struct SqPack {
    archive_paths: HashMap<SqPackArchiveId, PathBuf>,
    archives: HashMap<SqPackArchiveId, SqPackArchive>,
}

impl SqPack {
    pub fn new(base_dir: &Path) -> io::Result<SqPack> {
        let root_dirs = base_dir
            .read_dir()?
            .filter_map(Result::ok)
            .map(|x| x.path())
            .filter(|x| {
                let file_name = x.file_name().and_then(OsStr::to_str).unwrap();
                file_name == "ffxiv" || file_name.starts_with("ex")
            });

        let entries = root_dirs.flat_map(|x| {
            x.read_dir()
                .unwrap()
                .filter_map(Result::ok)
                .map(|y| y.path())
                .filter(|y| y.extension().and_then(OsStr::to_str).unwrap() == "index")
        });

        let archive_paths = entries
            .map(|x| (SqPack::get_archive_id(&x), x))
            .collect::<HashMap<_, _>>();

        Ok(SqPack {
            archive_paths,
            archives: HashMap::new(),
        })
    }

    fn get_archive(&mut self, archive_id: &SqPackArchiveId) -> &SqPackArchive {
        let archive_paths = &self.archive_paths;
        self.archives
            .entry(archive_id.to_owned())
            .or_insert_with(|| SqPackArchive::new(archive_paths.get(archive_id).unwrap()).unwrap())
    }

    fn get_archive_id(path: &Path) -> SqPackArchiveId {
        let file_name = path.file_stem().and_then(OsStr::to_str).unwrap();
        let archive_id_str = file_name.split('.').next().unwrap();
        let archive_id = u32::from_str_radix(archive_id_str, 16).unwrap();

        // ex) 0a0000
        let root = (archive_id / 0x10000) as u8;
        let ex = ((archive_id / 0x100) & 0xff) as u8;
        let part = (archive_id & 0xff) as u8;

        SqPackArchiveId { root, ex, part }
    }
}

impl Package for SqPack {
    fn read_file(&mut self, path: &str) -> io::Result<Vec<u8>> {
        let reference = SqPackFileReference::new(path);
        let archive = self.get_archive(&reference.archive_id);

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
        let mut pack = SqPack::new(Path::new(
            "D:\\Games\\FINAL FANTASY XIV - KOREA\\game\\sqpack",
        ))
        .unwrap();

        pack.read_file("exd/item.exh").unwrap();
        pack.read_file("bg/ex1/01_roc_r2/common/bgparts/r200_a0_bari1.mdl")
            .unwrap();
    }
}
