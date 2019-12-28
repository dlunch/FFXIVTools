use std::io::Result;
use std::path::Path;

pub trait Package {
    fn read_file(&self, filename: &Path) -> Result<Vec<u8>>;
}
