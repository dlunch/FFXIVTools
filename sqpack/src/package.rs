use std::io::Result;

pub trait Package {
    fn read_file(&self, filename: &str) -> Result<Vec<u8>>;
}