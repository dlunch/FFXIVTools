use std::io;

pub trait Package {
    fn read_file(&self, path: &str) -> io::Result<Vec<u8>>;
}
