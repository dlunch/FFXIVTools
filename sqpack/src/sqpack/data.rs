use std::fs::File;
use std::io;
use std::path::Path;

use super::ext::ReadExt;
use super::parser::FileHeader;

pub struct SqPackData {
    file: File,
}

impl SqPackData {
    pub fn new(path: &Path) -> io::Result<SqPackData> {
        let file = File::open(path)?;

        Ok(SqPackData { file })
    }

    pub fn read(&mut self, offset: u32) -> io::Result<Vec<u8>> {
        let file_header = read_and_parse!(self.file, offset, FileHeader);

        Ok(Vec::new())
    }
}
