use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

use super::archive::SqPackArchive;
use super::archive_id::SqPackArchiveId;

pub struct SqPackArchiveContainer {
    archive_paths: HashMap<SqPackArchiveId, PathBuf>,
    archives: HashMap<SqPackArchiveId, SqPackArchive>,
}

impl SqPackArchiveContainer {
    pub fn new(base_dir: &Path) -> io::Result<Self> {
        let root_dirs = base_dir.read_dir()?.filter_map(Result::ok).map(|x| x.path()).filter(|x| {
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

        let archive_paths = entries.map(|x| (SqPackArchiveId::with_sqpack_path(&x), x)).collect::<HashMap<_, _>>();

        Ok(Self {
            archive_paths,
            archives: HashMap::new(),
        })
    }

    pub async fn get_archive(&mut self, archive_id: &SqPackArchiveId) -> io::Result<&mut SqPackArchive> {
        let archive_paths = &self.archive_paths;
        if !self.archives.contains_key(archive_id) {
            let archive = SqPackArchive::new(archive_paths.get(archive_id).unwrap()).await?;
            self.archives.insert(*archive_id, archive);
        }
        Ok(self.archives.get_mut(archive_id).unwrap())
    }
}
