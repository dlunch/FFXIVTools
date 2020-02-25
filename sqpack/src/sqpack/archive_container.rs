use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::archive::SqPackArchive;
use crate::common::SqPackArchiveId;

pub struct SqPackArchiveContainer {
    archive_paths: HashMap<SqPackArchiveId, PathBuf>,
    archives: Mutex<HashMap<SqPackArchiveId, Arc<SqPackArchive>>>,
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
            archives: Mutex::new(HashMap::new()),
        })
    }

    // https://github.com/rust-lang/rust-clippy/issues/5176
    #[allow(clippy::map_entry)]
    pub async fn get_archive(&self, archive_id: SqPackArchiveId) -> io::Result<Arc<SqPackArchive>> {
        let mut archives = self.archives.lock().await;

        if !archives.contains_key(&archive_id) {
            let archive = SqPackArchive::new(self.archive_paths.get(&archive_id).unwrap()).await?;
            archives.insert(archive_id, Arc::new(archive));
        }

        let archive = archives.get(&archive_id).unwrap();

        Ok(archive.clone())
    }
}
