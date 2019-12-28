use std::io;
use std::path::Path;

pub trait Package {
    fn read_file(&self, path: &Path) -> io::Result<Vec<u8>>;
}
