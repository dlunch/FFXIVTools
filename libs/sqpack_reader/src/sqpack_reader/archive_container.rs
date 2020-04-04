use std::collections::HashMap;
use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use log::debug;
use tokio::sync::RwLock;

use super::archive::SqPackArchive;
use crate::archive_id::SqPackArchiveId;

pub struct SqPackArchiveContainer {
    archive_paths: HashMap<SqPackArchiveId, PathBuf>,
    archives: RwLock<HashMap<SqPackArchiveId, Arc<SqPackArchive>>>,
}

impl SqPackArchiveContainer {
    pub fn new(base_dir: &Path) -> io::Result<Self> {
        debug!("Opening {}", base_dir.to_str().unwrap());

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

        let archive_paths = entries
            .map(|x| (SqPackArchiveId::from_sqpack_file_name(x.file_stem().and_then(OsStr::to_str).unwrap()), x))
            .collect::<HashMap<_, _>>();

        Ok(Self {
            archive_paths,
            archives: RwLock::new(HashMap::new()),
        })
    }

    pub async fn get_archive(&self, archive_id: SqPackArchiveId) -> io::Result<Arc<SqPackArchive>> {
        {
            let archives = self.archives.read().await;
            if archives.contains_key(&archive_id) {
                return Ok(archives.get(&archive_id).unwrap().clone());
            }
        }

        let mut archives = self.archives.write().await;
        let archive = SqPackArchive::new(self.archive_paths.get(&archive_id).unwrap()).await?;
        archives.insert(archive_id, Arc::new(archive));

        Ok(archives.get(&archive_id).unwrap().clone())
    }
}
