use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom};

pub trait ReadExt {
    fn read_to_vec(&mut self, offset: u64, size: usize) -> io::Result<Vec<u8>>;
}

impl ReadExt for File {
    fn read_to_vec(&mut self, offset: u64, size: usize) -> io::Result<Vec<u8>> {
        let mut data = vec![0; size];
        self.seek(SeekFrom::Start(offset))?;
        self.read_exact(data.as_mut_slice())?;

        Ok(data)
    }
}
