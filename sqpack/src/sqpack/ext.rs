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

macro_rules! read_and_parse {
    ($file: expr, $offset: expr, $type: ty) => {{
        let data = $file.read_to_vec($offset as u64, <$type>::SIZE as usize)?;
        <$type>::parse(&data).unwrap().1
    }};
}
