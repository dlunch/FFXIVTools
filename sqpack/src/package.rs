use std::io;

pub trait Package {
    fn read_file(&mut self, path: &str) -> io::Result<Vec<u8>>;
}
